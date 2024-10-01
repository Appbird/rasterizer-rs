use crate::{snapshot, assert_cond};
use crate::util::{ClosedInterval, Color, Point2};
use super::{bresenham::BresenhamLine, Canvas};

impl Canvas {
    pub fn draw_triangle(
        &mut self,
        points: &[&Point2; 3],
        color: &Color
    ) {
        // y基準でソート
        let mut indecies: [usize; 3] = [0, 1, 2];
        indecies.sort_by(|a, b| points[a.clone()].y.cmp(&points[b.clone()].y));
        let [bottom, middle, top] = indecies.map(|i| points[i.clone()]);
        
        // 三角形の輪郭を求める
        let mut iter1 = [
            BresenhamLine::trace(bottom, middle),
            BresenhamLine::trace(middle, top),
        ];
        let mut iter1_idx = 0;
        let mut iter2 = BresenhamLine::trace(bottom, top);

        for y in bottom.y .. top.y{
            // println!("[loop] {y} in {} .. {}", bottom.y, top.y);
            // 次のイテレータのポイントを見出す
            let p1 =
                if let Some(x) = iter1[iter1_idx].next() { x } else {
                    assert!(iter1_idx == 0);
                    iter1_idx += 1;
                    iter1[iter1_idx].skip_to_next_y();
                    iter1[iter1_idx].next().unwrap()
                };
            let p2 =
                if let Some(x) = iter2.next() { x } else { break; };
            snapshot!(p1);
            snapshot!(p2);
            // それぞれのイテレータの辿っている点のy座標が`y`に達するまでスキップする
            let edge1 = iter1[iter1_idx].skip_to_next_y().unwrap();
            let edge2 = iter2.skip_to_next_y().unwrap();
            snapshot!(edge1);
            snapshot!(edge2);
            assert_cond!(p1.y, y);
            assert_cond!(p1.y, edge1.y);
            assert_cond!(p2.y, y);
            assert_cond!(p2.y, edge2.y);

            let before = ClosedInterval::between(p1.x, p2.x);
            let after = ClosedInterval::between(edge1.x, edge2.x);
            let draw_interval = before.or(after);
            for x in draw_interval {
                self.draw_pixel(&Point2::new(x, y), color);
            }
            
        }
    }
}