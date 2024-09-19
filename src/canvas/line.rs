use super::base::Canvas;
use super::bresenham::BresenhamLine;
use crate::util::{Point2, Color};


impl Canvas {
    /**
     * 点p1から点p2への線分をBresenhamの線分描画アルゴリズムによって描画する。
     * Color値のうち、Alpha値は無視される。
     */
    pub fn draw_line(&mut self, p1:&Point2, p2:&Point2, color: &Color) {
        for p in BresenhamLine::trace(p1, p2){
            self.draw_pixel(&p, color)
        }
    }
}