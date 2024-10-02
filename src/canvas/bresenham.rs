
use std::cmp::min;

use crate::{snapshot, util::Point2};


/**
 * 与えられたある2点の間を太さ1の線分で描いたときに通る画素の点を列挙するイテレータ。
 * 列挙方法はBresenhamの線分描画アルゴリズムに基づく。 
 */
#[derive(Debug)]
 pub struct BresenhamLineIter {
    /**
     * `true`: BresenhamのアルゴリズムがX軸に沿って動いている
     * （i.e. 線分の存在する範囲内の任意のx整数値に対して、ある値yがただ一つ存在し、点(x, y)がプロットされている。）
     * `false`: BresenhamのアルゴリズムがY軸に沿って動いている。この時、p1, p2のx値とy値が入れ替わって扱われていることに注意
     * （i.e. 線分の存在する範囲内の任意のy整数値に対して、ある値xがただ一つ存在し、点(x, y)がプロットされている。）
     */
    along_x_axis: bool,
    
    p2:Point2,
    /** このイテレータがどの点を指しているかを示す。 */
    current: Point2,
    next: Point2,
    // コメント中の解説は、along_x_axis以外、全てX軸にそってアルゴリズムが動いていることを仮定して説明している。
    
    /** p2 - p1 */
    offset: Point2,
    /** x値が正の方向に進むか、負の方向に進むかを表す。1, -1の値を取り、イテレータが一度走るたびにこの値がX値に加算される。 */
    x_direction: i32,
    /** y値が正の方向に進むか、負の方向に進むかを表す。1, -1の値を取り、`err`の誤差が0以上になるたびにこの値がY値に加算される。 */
    y_direction: i32,

    /** 現在描こうとしている点`current`と本来描こうとしている線分`Y`方向における累積誤差を表す。
     * より正確には、この値に対してスケーリングと並行移動を施したものであり、こうすることで整数範囲内での演算に収まるように工夫されている。
     * この値が0を超えるとy値にsが加算される。 */
     err:i32,
}
pub struct BresenhamLine();

impl BresenhamLine {
    /**
    制約
    - この関数から返されるイテレータは長さ1以上の系列を生み出す。
     */
    pub fn trace(p1:&Point2, p2:&Point2) -> BresenhamLineIter {
        // どの軸に沿って線形走査を行うかを表す。`true`ならばx軸に沿って線を描き、`false`ならy軸に沿って線を描く。
        let along_x_axis = (p2.x - p1.x).abs() > (p2.y - p1.y).abs();
        // |dy/dx| > 1であったときはy = xに対して対称に移動させる。点を打つ際にはちゃんと移動させた分を戻す。
        let flipped_points = (p1.flipped(), p2.flipped());
        let (p1, p2) = if along_x_axis { (p1.clone(), p2.clone()) } else { flipped_points };
        
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let offset = Point2::new(dx, dy);
        
        // 線形走査において、正の方向に移動するか、負の方向に移動するか
        let x_direction = dx.signum();
        // 誤差が大きくなった場合に描画する画素の対象を正の方向にずらすか負の方向にずらすか
        let y_direction = dy.signum();
        let current = p1.clone();
        let next = p1.clone();
        let err = -i32::abs(dx);
        BresenhamLineIter{
            p2,
            along_x_axis, current, next,
            x_direction, y_direction,
            offset, err, 
        }
    }
}

impl BresenhamLineIter {
    fn at(&self) -> Option<Point2> {
        let p = &self.current;
        if
            self.x_direction == 1  && p.x <= self.p2.x ||
            self.x_direction == -1 && p.x >= self.p2.x
        {
            // xyを反転させて処理していた場合には、点を打つ際にちゃんとxyを反転しなおした分を戻す。
            Some(if self.along_x_axis { p.clone() } else { p.flipped() })
        } else {
            None
        }
    }
}

impl Iterator for BresenhamLineIter {
    type Item = Point2;
    fn next(&mut self) -> Option<Self::Item> {
        self.current = self.next.clone();
        
        let p = &mut self.next;
        let dx = self.offset.x;
        let dy = self.offset.y;
        p.x += self.x_direction;
        self.err += 2 * i32::abs(dy);
        if self.err > 0 {
            p.y += self.y_direction;
            self.err -= 2 * i32::abs(dx);
        }
        snapshot!(self.current);
        snapshot!(self.next);
        self.at()
    }
}


impl BresenhamLineIter {
    /** y値が指定されたyになる直前までイテレータを飛ばす。 */
    // #TODO なにかまだバグがある
    pub fn skip_to_y(&mut self, y:i32) -> Option<Point2> {
        // Y軸に沿っていた場合は、その地点がすでにy値が更新される直前の点である。
        if self.next.y == y {
            return self.at();
        }
        
        let (step_x, step_y) = {
            // アルゴリズムが軸に沿う側(vertical)について、後どれだけ進められるか
            let left_steph= i32::abs(self.p2.x - self.current.x);
            // アルゴリズムが軸に沿わない側(horizontal)について、後どれだけ進められるか
            let left_stepv= i32::abs(self.p2.y - self.current.y);
            let left_stepx = if self.along_x_axis { left_steph } else { left_stepv };
            let left_stepy = if self.along_x_axis { left_stepv } else { left_steph };

            let dx = if self.along_x_axis { self.offset.x } else { self.offset.y };
            let dx = i32::abs(dx);
            let dy = if self.along_x_axis { self.offset.y } else { self.offset.x };
            let dy = i32::abs(dy);

            // yを何回進めるべきか
            let step_y = min(i32::abs(y - self.next.y), left_stepy);
            // xを何回進めるべきか
            let step_x = min((dx * step_y - 1) / dy, left_stepx);
            (step_x, step_y)
        };

        // アルゴリズムが沿った側の軸(horizontal)において1進むと、誤差がどれだけ変化するかを表す。
        let h1step_err = 2 * i32::abs(self.offset.x);
        // アルゴリズムが沿わなかった側の軸(vertical)において1進むと、誤差がどれだけ変化するかを表す。
        let v1step_err = 2 * i32::abs(self.offset.y);
        
        self.err += 
            if self.along_x_axis {
                h1step_err*step_x - v1step_err*step_y
            } else {
                h1step_err*step_y - v1step_err*step_x
            };
        
        self.next.x += step_x;
        self.next.y += step_y;
        
        self.current = self.next.clone();
        self.next()
    }
}
