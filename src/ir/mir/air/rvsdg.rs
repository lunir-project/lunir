use std::{borrow::Cow, marker::PhantomData};

use cranelift_entity::{entity_impl, PrimaryMap};
use tinyvec::TinyVec;

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct _VertexKey(u32);
entity_impl!(_VertexKey, "vertex_");

type VertexKey<'a> = Branded<'a, _VertexKey>;

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct ConstantKey(u32);
entity_impl!(ConstantKey, "constant_");

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct RegionKey(u32);
entity_impl!(RegionKey, "region_");

type Brand<'a> = PhantomData<fn(&'a ()) -> &'a ()>;

#[derive(Debug)]
struct Branded<'a, T> {
    _phantom: Brand<'a>,
    data: T,
}

impl<'a, T: Default> Default for Branded<'a, T> {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

enum ValTree<'a> {
    Leaf(i128),
    Branch(Cow<'a, [ConstantKey]>),
}

impl<'a> ValTree<'a> {
    #[inline]
    fn zst() -> Self {
        Self::Branch(Cow::Borrowed(&[]))
    }

    #[inline]
    fn unwrap_leaf(self) -> Option<i128> {
        match self {
            Self::Leaf(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    fn unwrap_branch(self) -> Option<Cow<'a, [ConstantKey]>> {
        match self {
            Self::Branch(b) => Some(b),
            _ => None,
        }
    }
}

enum ComplexVertices<'a> {
    Omega(RegionKey),
    Phi,
    Delta(ValTree<'a>),
    Theta,
    Gamma(TinyVec<[RegionKey; 2]>),
    Lambda,
}

enum SimpleVertices {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    Load,
    Store,
    Ud
}

enum VertexKind<'a> {
    Complex(ComplexVertices<'a>),
    Simple(SimpleVertices),
}

struct Vertex<'a> {
    kind: VertexKind<'a>,
    predecessors: TinyVec<[VertexKey<'a>; 3]>,
}

struct Edge<'a> {
    from: VertexKey<'a>,
    to: VertexKey<'a>,
}

struct Region<'a> {
    start: VertexKey<'a>,
    end: VertexKey<'a>,
}

#[derive(Default)]
struct Graph<'a> {
    vertices: PrimaryMap<_VertexKey, Vertex<'a>>,
    constants: PrimaryMap<ConstantKey, ValTree<'a>>,
    regions: PrimaryMap<RegionKey, Region<'a>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Self::default()
    }

    fn with_capacities(vertex_capacity: u32, constant_capacity: u32, region_capacity: u32) -> Self {
        Self {
            vertices: PrimaryMap::with_capacity(vertex_capacity as usize),
            constants: PrimaryMap::with_capacity(constant_capacity as usize),
            regions: PrimaryMap::with_capacity(region_capacity as usize),
        }
    }
    
    use VertexKind::*;
    use ComplexVertices::*;
    
    fn add_omega(&mut self, name: String)  {
        self.vertices.push(Vertex { kind: Complex(Omega()), predecessors: () }
    }

    // we need to make these constructors make an omega, unless you want to make that
    // a manual process to add an omega
    // i guess we should make it manual because you are allowed to have several for "linking"
    // dont forget we need a root region
    // right, we need to add to the omega variant a region key
    // i think we should replace the regions field with a root_region field.
    // bro what we need is to have a specific root region type (Omega's Region, ik,  but is the roo), even though that sounds kind of dumb, cuz listen
    // ik omega is region, but i'm thinking we scrap the vertices field, have a root_region field with a RootRegion type with the vertices
    // and the structural vertices(some of them obviously) can have their own regions, all the vertices are gonna be under the root anyway
    // the thing is that most rvsdgs that have been implemented dont have nodes containing direct references to others
    // because it's build bottom up and they use predecessor ids instead of successor references/ids
    // i dont know why exactly but i think it's a fine approach based on how the rvsdg is structured aka there's
    // usually more inputs than outputs, kinda like an upside down tree
    // that's so weird, are we sure we wanna follow that convention, i mean, do we have to? 
    // i feel
}
