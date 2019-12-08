struct ColumnIter<I> where I: Iterator {
    iterators: Vec<I>
}

impl<I, T> Iterator for ColumnIter<I> where I: Iterator<Item=T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iterators.iter_mut().map(|iter| iter.next()).collect()
    }
}

struct Image {
    layers : Vec<Vec<u32>>,
    width : usize
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.collapse().chunks(self.width) {
            for pixel in row {
                if *pixel == 1 {
                    write!(f, "☐")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl Image {
    fn new(input: &'static str, width: usize, height: usize) -> Image {
        Image {
            layers : input.chars()
                            .map(|c| c.to_digit(10).unwrap())
                            .collect::<Vec<_>>()
                            .chunks(width * height)
                            .map(|c| c.to_vec())
                            .collect(),
            width
        }
    }

    fn collapse(&self) -> Vec<u32> {
        let iterators : Vec<_> = (&self.layers).into_iter().map(|v| v.into_iter()).collect();
        let column_iter = ColumnIter { iterators };
        column_iter.map(|pixels| {
            //find first non-transparent pixel
            **(pixels.iter().filter(|p| ***p != 2).nth(0).unwrap())
        }).collect()
    }

    fn get_layers(&self) -> &Vec<Vec<u32>> {
        &self.layers
    }

    fn get_layer(&self, index: usize) -> &Vec<u32> {
        assert!(index < self.layers.len());
        &self.layers[index]
    }
}

fn calc_part_1(image: &Image) -> u32 {
    let (idx, _) = image.get_layers().iter().enumerate().map(|(i, layer)| {
        (i, layer.iter().filter(|p| **p == 0).count())
    }).min_by_key(|x| x.1).unwrap();

    let fewest_0_layer = image.get_layer(idx);

    fewest_0_layer.iter().filter(|p| **p == 1).count() as u32 * 
    fewest_0_layer.iter().filter(|p| **p == 2).count() as u32
}

fn main() {
    let image = Image::new(include_str!("../input/day_8.txt"), 25, 6);
    println!("Part 1 => {}", calc_part_1(&image));
    println!("Part 2 =>\n{}", image);
}

#[test]
fn build_layers() {
    let img = Image::new("123456789012", 3, 2);
    assert_eq!(*img.get_layers(), vec!(vec!(1,2,3,4,5,6), vec!(7,8,9,0,1,2)));
}

#[test]
fn part_1_complete() {
    let image = Image::new(include_str!("../input/day_8.txt"), 25, 6);
    assert_eq!(calc_part_1(&image), 1920);
}

#[test]
fn part_2() {
    let image = Image::new("0222112222120000", 2, 2);
    assert_eq!(image.collapse(), vec!(0,1,1,0));
}