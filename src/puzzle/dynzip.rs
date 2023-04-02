//
// This file is part of imgseek
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//

// https://stackoverflow.com/questions/54756166/how-do-i-efficiently-iterate-through-a-vecvect-row-by-row
pub struct DynamicZip<I>
where
    I: Iterator,
{
    pub iterators: Vec<I>,
}

impl<I, T> Iterator for DynamicZip<I>
where
    I: Iterator<Item = T>,
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let output: Option<Vec<T>> = self.iterators.iter_mut().map(|iter| iter.next()).collect();
        output
    }
}
