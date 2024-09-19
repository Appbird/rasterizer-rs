use crate::util::{ClosedInterval, Color, Point2};

use super::{bresenham::{BresenhamLine, BresenhamLineIter}, Canvas};

impl Canvas {
    fn triangle(
        &mut self,
        points: &mut [Point2; 3],
        color: &Color
    ) {
        let mut indecies: [usize; 3] = [0, 1, 2];
        indecies.sort_by(|a, b| points[a.clone()].y.cmp(&points[b.clone()].y));
        let [bottom, middle, top] = indecies.map(|i| points[i].clone());
        
        let mut iter1 = BresenhamLine::trace(&bottom, &middle);
        let mut iter2 = BresenhamLine::trace(&bottom, &top);
        let edge1 = iter1.next();
        let edge2 = iter2.next();
        // #TODO 三角形を描く
        for y in bottom.y .. middle.y{
            let edge1_p = edge1.unwrap();
            let edge2_p = edge2.unwrap();
            // それぞれのイテレータの辿っている点のy座標が`y`に達するまでスキップする
        
            
        }
    }
}