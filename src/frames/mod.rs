use bytes::{Buf, BufMut};

mod tests;

#[derive(PartialEq, Eq, Debug)]
pub enum Operation{
    Get,
    Set,
    Unknown
}

#[derive(PartialEq, Eq, Debug)]
pub struct Frame{
    pub body : Vec<i32>,
    pub op : Operation
}

impl Frame{
    pub fn new(body : Vec<i32>, op : Operation) -> Frame{
        Frame { body, op }
    }


    ///
    /// ```
    /// use http_server::frames::{Frame, Operation};
    /// use bytes::BufMut;
    ///  
    /// let f = Frame::new(vec![-2,1], Operation::Set);
    /// 
    /// let mut buf = vec![];
    /// let length = 2;
    /// let op = 0;
    /// let temp = length << 2 | op;
    /// buf.put_u16(temp);
    /// buf.put_i32((-2 as i32).to_be());
    /// buf.put_i32((1 as i32).to_be());
    /// 
    /// assert_eq!(buf, f.to_bytes());
    /// ```
    /// 
    pub fn to_bytes(self) -> Vec<u8>{
        let length : u16 = self.body.len() as u16;
        let opcode = Frame::get_opcode(self.op);

        let temp : u16 = length << 2 | opcode;

        let mut buf = vec![];
        buf.put_u16(temp);
        for i in 0..length{
            let t = self.body[i as usize];
            buf.put_i32(t.to_be());
        }

        buf
    }

    /// ```
    /// let mut buf = vec![];
    /// use http_server::frames::{Frame, Operation};
    /// use bytes::BufMut;
    ///
    /// let length = 2;
    /// let op = 0;
    /// let temp = length << 2 | op;
    ///
    /// buf.put_u16(temp);
    ///
    /// buf.put_i32((-2 as i32).to_be());
    /// buf.put_i32((1 as i32).to_be());
    ///
    /// let f = Frame::from_bytes(buf.as_ref());
    ///
    /// assert_eq!(f.op, Operation::Set);
    /// assert_eq!(f.body.len(), 2);
    /// assert_eq!(f.body, vec![-2,1]);
    /// ```

    pub fn from_bytes<T>(mut bytes : T) -> Frame
    where T : Buf
    {
        let temp = u16::from_be(bytes.get_u16());
        let length = (temp & 0xFFFC) >> 2;
        let opcode : u8 = (temp & 0b11) as u8;
        let length = u16::from_be(length);
        
        let op = Frame::get_op(opcode);

        let body = Frame::parse_body(bytes, length);

        Frame{body, op}
    }

    fn get_op(opcode : u8) -> Operation{
        use Operation::{Set, Get, Unknown};

        match opcode {
            0 => Set,
            1 => Get,
            _ => Unknown
        }
    }

    fn get_opcode(op : Operation) -> u16{
        use Operation::{Set, Get, Unknown};

        match op {
            Set => 0,
            Get => 1,
            Unknown => 64
        }
    }

    pub fn parse_body<T>(mut bytes : T, length : u16) -> Vec<i32>
    where T : Buf{
        let mut body = Vec::new();
        for i in 0..length{
            let number = bytes.get_i32();
            body.push(i32::from_be(number));
        }
        body
    }
}
