pub struct Network{
    leyers: Vec<Layer>,
}

struct Layer{
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Network{
    pub fn propagate(&self, mut inputs: Vec<f32>)-> Vec<f32>{\

        for layer in &self.leyers{
            inputs = layer.propagate(inputs);
        }

        inputs
    }
}

impl Layer{
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32>{
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

int get_berry_number(){
    //TODO
    return ();
}

fn berry_number()->usize{
    //TODO
    return();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
