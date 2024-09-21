use crate::util::{ClosedInterval, Color, Point2};

use super::{bresenham::BresenhamLine, Canvas};

impl Canvas {
    fn triangle(
        &mut self,
        points: &mut [Point2; 3],
        color: &Color
    ) {
        let mut indecies: [usize; 3] = [0, 1, 2];
        indecies.sort_by(|a, b| points[a.clone()].y.cmp(&points[b.clone()].y));
        let [bottom, middle, top] = indecies.map(|i| points[i].clone());
        let mut iter1 = [BresenhamLine::trace(&bottom, &middle), BresenhamLine::trace(&middle, &top)];
        let mut iter1_idx = 0;
        let mut iter2 = BresenhamLine::trace(&bottom, &top);
        // #TODO 三角形を描く
        for y in bottom.y .. top.y{
            // 次のイテレータのポイントを見出す
            let p1 =
                if let Some(x) = iter1[iter1_idx].next() { x } else {
                    iter1_idx += 1;
                    iter1[iter1_idx].skip_to_next_y();
                    iter1[iter1_idx].next().unwrap()
                };
            let p2 = if let Some(x) = iter2.next() { x } else { break; };
            // それぞれのイテレータの辿っている点のy座標が`y`に達するまでスキップする
            let edge1 = iter1[iter1_idx].skip_to_next_y().unwrap();
            let edge2 = iter2.skip_to_next_y().unwrap();
            let before = ClosedInterval::between(p1.x, p2.x);
            let after = ClosedInterval::between(edge1.x, edge2.x);
            let draw_interval = before.or(after);
            for x in draw_interval {
                self.draw_pixel(&Point2::new(x, y), color);
            }
            
        }
    }
}