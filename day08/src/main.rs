    const width:usize = 25;
    const height:usize = 6;

fn main() {
    let input: &str = include_str!("./input.txt").lines().next().unwrap();
    let data: Vec<isize> = input.chars().map(|x| x.to_digit(10).unwrap() as isize).collect();

    let mut layer = 0;
    let total_layers = data.len() / width / height;
    let mut layers: Vec<&[isize]> = vec![];
    let mut least_zeros = 99999999;
    let mut zero_layer = 0;
    let mut final_layer: Vec<isize> = vec![2; width * height];

    for layer in 0..total_layers {
        let start = layer * height * width;
        let end = start + (height * width);
        let slice = &data[start..end];
        layers.push(slice);

        let zero_count: usize = slice.iter().filter(|&x| *x == 0).collect::<Vec<&isize>>().len();
        if zero_count <= least_zeros {
            least_zeros = zero_count;
            zero_layer = layer;
        }
    }


    for (c, layer) in layers.iter().rev().enumerate() {
        for (i, &pixel) in layer.iter().enumerate() {
            if pixel < 2 {
                final_layer[i] = pixel;
            }
        }
    }

    let total_ones = layers[zero_layer].iter().filter(|&x| *x == 1).collect::<Vec<&isize>>().len();
    let total_twos = layers[zero_layer].iter().filter(|&x| *x == 2).collect::<Vec<&isize>>().len();
    println!("1: {} 2: {} Total: {}", total_ones, total_twos, total_twos * total_ones);

}

fn print_layer(final_layer: &Vec<isize>) {
    let items: Vec<char> = final_layer.iter().map(|&x| {
        return match x {
            0 => ' ',
            1 => 'X',
            _ => ' '
        }
    }).collect();

    for i in 0..height {
        let start = width * i;
        let end = start + width;
        println!("{}", &items[start..end].iter().collect::<String>());
    }
}
