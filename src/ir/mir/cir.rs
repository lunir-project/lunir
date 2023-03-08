use std::collections::HashMap;

use petgraph::{
    data::Build,
    dot::{Config, Dot},
    graph::NodeIndex,
    prelude::DiGraph,
    stable_graph::IndexType,
    Direction,
};

use crate::prelude::{IlChunk, Instruction, Jump};

#[derive(Default)]
pub(crate) struct CirGraph(DiGraph<IlChunk, bool, usize>);

impl CirGraph {
    pub(crate) fn inner(&self) -> &DiGraph<IlChunk, bool, usize> {
        &self.0
    }

    pub(crate) fn inner_mut(&mut self) -> &mut DiGraph<IlChunk, bool, usize> {
        &mut self.0
    }

    pub(crate) fn new() -> Self {
        Self::default()
    }
}

fn graph_get_or_insert(
    graph: &mut DiGraph<IlChunk, bool, usize>,
    weight: IlChunk,
) -> NodeIndex<usize> {
    match graph
        .node_weights()
        .zip(graph.node_indices())
        .find(|&(node_weight, _)| node_weight == &weight)
    {
        Some((_, idx)) => idx,
        None => graph.add_node(weight).index().into(),
    }
}

fn split_blocks(
    blocks: &HashMap<usize, &[Instruction]>,
    src_block_index: usize,
    target_block_index: usize,
    graph: &mut DiGraph<IlChunk, bool, usize>,
    is_negated: bool,
) -> NodeIndex<usize> {
    let src_block = blocks.get(&src_block_index).unwrap();
    for (&block_index, &curr_block) in blocks
        .iter()
        .filter(|&(&block_index, _)| block_index < target_block_index)
    {
        if (block_index..block_index + src_block.len()).contains(&target_block_index) {
            let relative_target_index = target_block_index - block_index;

            let src_node_index = graph_get_or_insert(graph, IlChunk::from(*src_block));

            let leading_block_index =
                graph_get_or_insert(graph, IlChunk::from(&curr_block[..relative_target_index]));

            let trailing_block_index =
                graph_get_or_insert(graph, IlChunk::from(&curr_block[relative_target_index..]));

            graph.add_edge(
                leading_block_index.into(),
                trailing_block_index.into(),
                true && !is_negated,
            );

            graph.add_edge(src_node_index.into(), trailing_block_index.into(), true);

            return src_node_index;
        }
    }

    panic!("Branch attempts to jump to invalid location");
}

pub(crate) fn into_cir_graph(instructions: Vec<Instruction>) {
    let mut blocks: HashMap<usize, &[Instruction]> = HashMap::new();

    let mut start = 0;

    for (index, instruction) in instructions.iter().enumerate() {
        if let Instruction::Jump(_) | Instruction::ConditionalJump(_) | Instruction::JumpNot(_) =
            *instruction
        {
            blocks.insert(start, &instructions[start..index + 1]);
            start = index + 1;
        }
    }

    blocks.insert(start, &instructions[start..]);

    // println!("{blocks:#?}");

    let mut cfg = CirGraph::new();

    let graph = cfg.inner_mut();

    for (pc, src_block) in blocks.iter() {
        match src_block.last() {
            Some(Instruction::Jump(data)) => {
                let target_block_index = data.branch.end;

                match blocks.get(&target_block_index) {
                    Some(target_block) => {
                        let from_block = graph_get_or_insert(graph, IlChunk::from(*src_block));

                        let to_block = graph_get_or_insert(graph, IlChunk::from(*target_block));

                        graph.add_edge(from_block, to_block, true);
                    }
                    None => {
                        split_blocks(&blocks, *pc, target_block_index, graph, false);
                    }
                };
            }

            Some(Instruction::ConditionalJump(data)) => {
                let target_block_index = data.branch.end;

                let source = match blocks.get(&target_block_index) {
                    Some(target_block) => {
                        let original_source = graph_get_or_insert(graph, IlChunk::from(*src_block));

                        let to = graph_get_or_insert(graph, IlChunk::from(*target_block));

                        graph.add_edge(original_source.into(), to.into(), true);

                        original_source
                    }
                    None => {
                        let original_source =
                            split_blocks(&blocks, *pc, target_block_index, graph, false);

                        original_source
                    }
                };

                let non_divergent = blocks.get(&(data.branch.start + 1)).unwrap();

                let next = graph_get_or_insert(graph, IlChunk::from(*non_divergent));

                graph.add_edge(source, next, false);
            }

            Some(Instruction::JumpNot(data)) => {
                let target_block_index = data.branch.end;

                let source = match blocks.get(&target_block_index) {
                    Some(target_block) => {
                        let original_source = graph_get_or_insert(graph, IlChunk::from(*src_block));

                        let to = graph_get_or_insert(graph, IlChunk::from(*target_block));

                        graph.add_edge(original_source.into(), to.into(), false);

                        original_source
                    }
                    None => {
                        let original_source =
                            split_blocks(&blocks, *pc, target_block_index, graph, true);

                        original_source.into()
                    }
                };

                let non_divergent = blocks.get(&(data.branch.start + 1)).unwrap();

                let next = graph_get_or_insert(graph, IlChunk::from(*non_divergent));

                graph.add_edge(source, next, true);
            }
            _ => continue,
        }
    }

    let graph = cfg.inner();

    // dbg!(graph);

    println!("{:#?}", Dot::with_config(graph, &[]));
}

impl From<IlChunk> for CirGraph {
    fn from(chunk: IlChunk) -> Self {
        let mut cfg = CirGraph::new();

        Self::default()
    }
}
