#[cfg(test)]
mod tests_frame {
    use bytes::BufMut;
    use crate::frames::{Frame, Operation};

    #[test]
    fn from_bytes_test() {

        let mut buf = vec![];

        let length = 2;
        let op = 0;
        let temp = length << 2 | op;

        buf.put_u16(temp);

        buf.put_i32((-2 as i32).to_be());
        buf.put_i32((1 as i32).to_be());

        let f = Frame::from_bytes(buf.as_ref());

        assert_eq!(f.op, Operation::Set);
        assert_eq!(f.body.len(), 2);
        assert_eq!(f.body, vec![-2,1]);
    }

    #[test]
    fn test_to_bytes(){
        let f = Frame::new(vec![-2,1], Operation::Set);

        let mut buf = vec![];

        let length = 2;
        let op = 0;
        let temp = length << 2 | op;
        buf.put_u16(temp);
        buf.put_i32((-2 as i32).to_be());
        buf.put_i32((1 as i32).to_be());

        assert_eq!(buf, f.to_bytes());
    }

    #[test]
    fn reciprocity(){
        let s =  Frame::new(vec![-2,1], Operation::Set);
        let f = Frame::new(vec![-2,1], Operation::Set);

        let mut buf = f.to_bytes();

        let f = Frame::from_bytes(buf.as_ref());

        assert_eq!(f, s);
    }
}