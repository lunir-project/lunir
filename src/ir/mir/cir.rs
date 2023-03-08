use std::{collections::HashMap};

use petgraph::{
    dot::{Config, Dot},
    prelude::DiGraph,
};

use crate::prelude::{IlChunk, Instruction};

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

    for (pc, block) in blocks.iter() {
        match (block.last(), block.len()) {
            (Some(Instruction::Jump(data)), len) => {
                let target = data.branch.end;

                match blocks.get(&target) {
                    Some(target_block) => {
                        let from = graph
                            .add_node(IlChunk::from(
                                *blocks.get(&dbg!(pc)).unwrap(),
                            ))
                            .index();
                        let to = graph.add_node(IlChunk::from(*target_block)).index();

                        graph.add_edge(from.into(), to.into(), true);
                    }
                    None => {
                        blocks
                            .iter()
                            .filter(|&(&k, _)| k < target)
                            .find_map(|(&k, &v)| {
                                if dbg!(dbg!(k..k + len).contains(dbg!(&target))) {
                                    let relative_target_index = target - k;

                                    let original_source =
                                        graph.add_node(IlChunk::from(*block)).index();

                                    let leading_block = graph
                                        .add_node(IlChunk::from(&v[..relative_target_index]))
                                        .index();

                                    let trailing_block = graph
                                        .add_node(IlChunk::from(&v[relative_target_index..]))
                                        .index();

                                    graph.add_edge(
                                        leading_block.into(),
                                        trailing_block.into(),
                                        true,
                                    );

                                    graph.add_edge(
                                        original_source.into(),
                                        trailing_block.into(),
                                        true,
                                    );

                                    Some(())
                                } else {
                                    None
                                }
                            })
                            .expect("Branch attempts to jump to invalid location");
                    }
                };
            }
            (Some(Instruction::ConditionalJump(data)), len) => {
                let target = data.branch.end;

                match blocks.get(&target) {
                    Some(target_block) => {
                        let from = graph
                            .add_node(IlChunk::from(
                                *blocks.get(&dbg!(pc)).unwrap(),
                            ))
                            .index();
                        let to = graph.add_node(IlChunk::from(*target_block)).index();

                        graph.add_edge(from.into(), to.into(), true);

                        let non_divergent = blocks.get(&(data.branch.start + 1)).unwrap();

                        let next = graph.add_node(IlChunk::from(*non_divergent)).index();

                        graph.add_edge(from.into(), next.into(), false);
                    }
                    None => {
                        blocks
                            .iter()
                            .filter(|&(&k, _)| k < target)
                            .find_map(|(&k, &v)| {
                                

                                if dbg!(dbg!(k..k + len).contains(dbg!(&target))) {
                                    let relative_target_index = target - k;

                                    let original_source =
                                        graph.add_node(IlChunk::from(*block)).index();

                                    let leading_block = graph
                                        .add_node(IlChunk::from(&v[..relative_target_index]))
                                        .index();

                                    let trailing_block = graph
                                        .add_node(IlChunk::from(&v[relative_target_index..]))
                                        .index();

                                    graph.add_edge(
                                        leading_block.into(),
                                        trailing_block.into(),
                                        true,
                                    );

                                    graph.add_edge(
                                        original_source.into(),
                                        trailing_block.into(),
                                        true,
                                    );

                                    let non_divergent =
                                        blocks.get(&(data.branch.start + 1)).unwrap();

                                    let next =
                                        graph.add_node(IlChunk::from(*non_divergent)).index();

                                    graph.add_edge(original_source.into(), next.into(), false);

                                    Some(())
                                } else {
                                    None
                                }
                            })
                            .expect("Branch attempts to jump to invalid location");
                    }
                };
            }
            (Some(Instruction::JumpNot(data)), len) => {
                let target = data.branch.end;

                match blocks.get(&target) {
                    Some(target_block) => {
                        let from = graph
                            .add_node(IlChunk::from(
                                *blocks.get(&(pc)).unwrap(),
                            ))
                            .index();
                        let to = graph.add_node(IlChunk::from(*target_block)).index();

                        graph.add_edge(from.into(), to.into(), false);

                        let non_divergent = blocks.get(&(data.branch.start + 1)).unwrap();

                        let next = graph.add_node(IlChunk::from(*non_divergent)).index();

                        graph.add_edge(from.into(), next.into(), true);
                    }
                    None => {
                        blocks
                            .iter()
                            .filter(|&(&k, _)| k < target)
                            .find_map(|(&k, &v)| {
                                if dbg!(dbg!(k..k + len).contains(dbg!(&target))) {
                                    let relative_target_index = target - k;

                                    let original_source =
                                        graph.add_node(IlChunk::from(*block)).index();

                                    let leading_block = graph
                                        .add_node(IlChunk::from(&v[..relative_target_index]))
                                        .index();

                                    let trailing_block = graph
                                        .add_node(IlChunk::from(&v[relative_target_index..]))
                                        .index();

                                    graph.add_edge(
                                        leading_block.into(),
                                        trailing_block.into(),
                                        true,
                                    );

                                    graph.add_edge(
                                        original_source.into(),
                                        trailing_block.into(),
                                        false,
                                    );

                                    let non_divergent =
                                        blocks.get(&(data.branch.start + 1)).unwrap();

                                    let next =
                                        graph.add_node(IlChunk::from(*non_divergent)).index();

                                    graph.add_edge(original_source.into(), next.into(), true);

                                    Some(())
                                } else {
                                    None
                                }
                            })
                            .expect("Branch attempts to jump to invalid location");
                    }
                };
            }
            _ => continue,
        }
    }

    let graph = cfg.inner();

    // dbg!(graph);

    println!(
        "{:#?}",
        Dot::with_config(graph, &[])
    );
}

impl From<IlChunk> for CirGraph {
    fn from(chunk: IlChunk) -> Self {
        let mut cfg = CirGraph::new();

        let graph = cfg.inner_mut();

        let instructions = chunk.inner();

        Self::default()
    }
}
