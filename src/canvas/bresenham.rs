use crate::util::Point2;

/**
 * 与えられたある2点の間を太さ1の線分で描いたときに通る画素の点を列挙するイテレータ。
 * 列挙方法はBresenhamの線分描画アルゴリズムに基づく。
 */
struct BresenhamLineIter {
    /**
     * `true`: BresenhamのアルゴリズムがX軸に沿って動いている
     * （i.e. 線分の存在する範囲内の任意のx整数値に対して、ある値yが存在し、ただ一つだけ点(x, y)がプロットされている。）
     * `false`: BresenhamのアルゴリズムがY軸に沿って動いている。この時、p1, p2のx値とy値が入れ替わって扱われていることに注意
     * （i.e. 線分の存在する範囲内の任意のy整数値に対して、ある値xが存在し、ただ一つだけ点(x, y)がプロットされている。）
     */
    along_x_axis: bool,
    
    /** このイテレータがどの点を指しているかを示す。 */
    current: Point2,
    /** X値の上限を示す。 */
    max_x: i32,
    // コメント中の解説は、along_x_axis以外、全てX軸にそってアルゴリズムが動いていることを仮定して説明している。
    
    /** p2 - p1 */
    offset: Point2,
    /** y値が正の方向に進むか、負の方向に進むかを表す。1, -1の値を取り、イテレータが一度走るたびにこの値がsのX値に加算される。 */
    s: i32,

    /** 現在描こうとしている点`current`と本来描こうとしている線分`Y`方向における累積誤差を表す。
     * より正確には、この値に対してスケーリングと並行移動を施したものであり、こうすることで整数範囲内での演算に収まるように工夫されている。
     * この値が0を超えるとy値にsが加算される。 */
     err:i32,
}
     
impl BresenhamLineIter {
    fn trace(p1:Point2, p2:Point2) -> BresenhamLineIter {
        // どの軸に沿って線形走査を行うかを表す。`true`ならばx軸に沿って線を描き、`false`ならy軸に沿って線を描く。
        let along_x_axis = (p2.x - p1.x).abs() > (p2.y - p1.y).abs();
        // 走査する軸の方向に対して、開始点と到着点が真逆になっていたときはp1とp2を入れ替えておく。
        if along_x_axis && p1.x > p2.x || !along_x_axis && p1.y > p2.y  {
            return BresenhamLineIter::trace(p2, p1);
        }
        // |dy/dx| > 1であったときはy = xに対して対称に移動させる。点を打つ際にはちゃんと移動させた分を戻す。
        let flipped_points = (p1.flipped(), p2.flipped());
        let (p1, p2) = if along_x_axis { (p1, p2) } else { flipped_points };
        
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        assert!(dy.abs() < dx.abs()); // |dy/dx| < 1
        let offset = Point2::new(dx, dy);
        
        // 誤差が大きくなった場合に描画する画素の対象を正の方向にずらすか負の方向にずらすか
        let s = if dy > 0 { 1 } else { -1 };
        let current = p1;
        let max_x = p2.x;
        let err = -dx;

        BresenhamLineIter{
            along_x_axis,
            current, max_x,
            s, offset,
            err, 
        }
    }
}

impl Iterator for BresenhamLineIter {
    type Item = Point2;
    fn next(&mut self) -> Option<Self::Item> {
        let p = &mut self.current;
        let dx = self.offset.x;
        let dy = self.offset.y;
        p.x += 1;
        if p.x <= self.max_x {
            self.err += 2 * i32::abs(dy);
            if self.err > 0 {
                p.y += self.s;
                self.err -= 2 * dx;
            }
            // 反転させて処理していた場合には、点を打つ際にちゃんとx, yを反転させた分を戻す。
            Some(if self.along_x_axis { self.current.clone() } else { self.current.flipped() })
        } else {
            None
        }
    }
}