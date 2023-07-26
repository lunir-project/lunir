
// TODO: remove once everything is used
#![allow(unused)]

use std::collections::HashMap;

use petgraph::{dot::Dot, graph::NodeIndex, prelude::DiGraph, visit::EdgeRef};

use crate::ir::il::{IlChunk, Instruction};

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
    removals: &mut Vec<(usize, usize, usize)>,
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

            removals.push((
                target_block_index.into(),
                leading_block_index.index(),
                trailing_block_index.index(),
            ));

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

    let mut to_be_removed = vec![];

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
                        split_blocks(
                            &blocks,
                            *pc,
                            target_block_index,
                            graph,
                            &mut to_be_removed,
                            false,
                        );
                    }
                };
            }

            Some(Instruction::ConditionalJump(data)) => {
                let target_block_index = data.branch.end;

                let source = match blocks.get(&target_block_index) {
                    Some(target_block) => {
                        let src_node = graph_get_or_insert(graph, IlChunk::from(*src_block));

                        let to_node = graph_get_or_insert(graph, IlChunk::from(*target_block));

                        graph.add_edge(src_node.into(), to_node.into(), true);

                        src_node
                    }
                    None => {
                        let src_node = split_blocks(
                            &blocks,
                            *pc,
                            target_block_index,
                            graph,
                            &mut to_be_removed,
                            false,
                        );

                        src_node
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
                        let original_source = split_blocks(
                            &blocks,
                            *pc,
                            target_block_index,
                            graph,
                            &mut to_be_removed,
                            true,
                        );

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

    for (block_index, leading_block_index, trailing_block_index) in to_be_removed {
        let mut edge_data = vec![];

        for edge in graph.edges(block_index.into()) {
            edge_data.push((
                edge.id(),
                edge.source().index(),
                edge.target().index(),
                edge.weight().clone(),
            ));
        }

        for datum in edge_data {
            let (index, source, target, weight) = datum;

            graph.remove_edge(index.into());

            if source == block_index {
                if target == source {
                    dbg!(graph.node_weight(block_index.into()));
                    graph
                        .add_edge(
                            trailing_block_index.into(),
                            dbg!(leading_block_index.into()),
                            weight,
                        )
                        .index();
                } else {
                    graph.add_edge(trailing_block_index.into(), dbg!(target.into()), weight);
                }
            } else {
                graph.add_edge(source.into(), leading_block_index.into(), weight);
            }

            graph.remove_edge(index);
        }

        graph.remove_node(block_index.into());
    }

    let graph = cfg.inner();

    // dbg!(graph);

    println!("{:#?}", Dot::with_config(graph, &[]));
}

impl From<IlChunk> for CirGraph {
    fn from(_chunk: IlChunk) -> Self {
        let _cfg = CirGraph::new();

        Self::default()
    }
}
