use super::base::Canvas;
use crate::util::{Point2, Color, in_range};

impl Canvas {
    /**
     * 点p1から点p2への線分をBresenhamの線分描画アルゴリズムによって描画する。
     * Color値のうち、Alpha値は無視される。
     */
    pub fn draw_line(&mut self, p1:&Point2, p2:&Point2, color: &Color) {
        // どの軸に沿って線形走査を行うかを表す。`true`ならばx軸に沿って線を描き、`false`ならy軸に沿って線を描く。
        let along_x_axis = (p2.x - p1.x).abs() > (p2.y - p1.y).abs();
        // 走査する軸の方向に対して、開始点と到着点が真逆になっていたときはp1とp2を入れ替えておく。
        if along_x_axis && p1.x > p2.x || !along_x_axis && p1.y > p2.y  {
            return self.draw_line(p2, p1, color);
        }
        // |dy/dx| > 1であったときはy = xに対して対称に移動させる。点を打つ際にはちゃんと移動させた分を戻す。
        let flipped_points = (&p1.flipped(), &p2.flipped());
        let (p1, p2) = if along_x_axis { (p1, p2) } else { flipped_points };

        // Bresenhamのアルゴリズムの実装（ただし、|dy/dx| < 1で実行できるように拡張）
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        assert!(dy.abs() < dx.abs()); // |dy/dx| < 1
        // 誤差が大きくなった場合に描画する画素の対象を正の方向にずらすか負の方向にずらすか
        let s = if dy > 0 { 1 } else { -1 };
        let mut y = p1.y;
        let mut err = -dx;
        for x in p1.x..p2.x {
            err += 2 * i32::abs(dy);
            if err > 0 {
                y += s;
                err -= 2 * dx;
            }
            // 点を打つ際にはちゃんと対称に移動させた分を戻す。
            let (x, y) = if along_x_axis {(x, y)} else {(y, x)};
            if in_range(0, x, self.width as i32) && in_range(0, y, self.height as i32) {
                self.draw_pixel(x as usize, y as usize, color)
            }
        }
    }
}