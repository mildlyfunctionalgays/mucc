pub struct WhitespaceSkip<'a, It: Iterator<Item=char>> {
    it: &'a mut It
}

impl<'a, It> WhitespaceSkip<'a, It>
where It: Iterator<Item=char>{
    pub fn new(it: &'a mut It) -> WhitespaceSkip<It> {
        WhitespaceSkip {it}
    }
}

impl<'a, It> Iterator for WhitespaceSkip<'a, It>
    where It: Iterator<Item=char> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.it.take_while(|ch| " \t\n".chars().any(|c| c == *ch)).last()
    }
}