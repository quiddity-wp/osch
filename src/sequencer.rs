use std::{ iter::IntoIterator, rc::Rc};

/// The base unit of sound : Either a duration of silence, or a duration and note.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Particle {
    /// A musical note to be played
    /// 
    /// `dur` : duration in nanoseconds
    /// `note`: midi note in 0-127
    Note {
        dur: u64,
        pitch: u8,
    },
    /// A silence to be held
    /// 
    /// `dur` : duration in nanoseconds
    Silence { 
        dur: u64
    },
}

/// A sequencer parsed instruction. An [Atom] refers to instructions that can contain
/// [Particle]s.
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Atom {
    /// A single particle
    Singleton(Particle),
    Cycle{
        atoms : Vec<Atom>, 
        len : u8,
        times : u8
    }
}

impl Atom {
    /// If the atom is a singleton, returns the contained particle.
    /// Returns `None` otherwise
    pub fn particle_ref(&self) -> Option<&Particle> {
        match self {
            Atom::Singleton(particle) => Some(particle),
            _ => None
        }
    }
    /// If the atom is a cycle, return a slice into its atoms.
    /// Returns `None` otherwise.
    pub fn atoms_slice(&self) -> Option<&[Atom]> {
        match self {
            Atom::Cycle { atoms, len : _, times : _ } => Some(atoms.as_slice()),
            _ => None
        }
    }

    fn iter<'a, 'b : 'a>(&'b self) -> Box<dyn Iterator<Item=&'a Particle> + 'a> {
        match self {
            Atom::Singleton(particle) => Box::new(std::iter::once(particle)),
            Atom::Cycle{times, len : _, atoms : _} => {
                // TODO can probably be improved
                let particles : Vec<_> = self.atoms_slice().unwrap().iter()
                .map(|x| x.iter()).flatten().collect();

                return Box::new(
                    particles.repeat(*times as usize).into_iter()
                );
            }
        }
    }
}











#[cfg(test)]
mod tests {
    use super::*;

    const A : Particle = Particle::Note{dur:1,pitch:2};
    const B : Particle = Particle::Note{dur:2,pitch:2};
    const C : Particle = Particle::Note{dur:3,pitch:2};
    const D : Particle = Particle::Note{dur:4,pitch:2};

    fn test_case() -> Atom {
        #[allow(non_snake_case)]
        
        // loop3 - A - B - loop2 - C - D
        Atom::Cycle{
            len: 3,
            times: 3,
            atoms: vec![
                Atom::Singleton(A),
                Atom::Singleton(B),
                Atom::Cycle{
                    len: 2,
                    times: 4,
                    atoms: vec![
                        Atom::Singleton(C),
                        Atom::Singleton(D),
                    ]
                }
            ]
        }
    }

    #[test]
    fn test_atom() {
        let case = test_case();
        assert_eq!(
            case.atoms_slice().unwrap()[0..2],
            [Atom::Singleton(A),Atom::Singleton(B)]
        );
    }
    #[test]
    fn print_case_iter() {
        let case = test_case();
        for x in case.iter() {
            println!("{:?}", x);
        }
    }
    
}