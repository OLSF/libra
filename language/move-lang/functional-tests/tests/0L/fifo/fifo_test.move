//! account: alice, 1000000, 0, validator

//! new-transaction
//! sender: alice
script {
    use 0x1::FIFO;
    fun main(_s: signer) {
        let f = FIFO::empty<u64>();
        let len = FIFO::len<u64>(& f);
        assert(len == 0, 1);

        FIFO::push<u64>(&mut f, 1);
        FIFO::push<u64>(&mut f, 2);
        FIFO::push<u64>(&mut f, 3);
        FIFO::push<u64>(&mut f, 4);
        FIFO::push<u64>(&mut f, 5);
        FIFO::push<u64>(&mut f, 6);
        FIFO::push<u64>(&mut f, 7);

        let len = FIFO::len<u64>(& f);
        assert(len == 7, 1);

        let a = FIFO::pop<u64>(&mut f); //1
        let b = FIFO::pop<u64>(&mut f); //2
        let c = FIFO::pop<u64>(&mut f); //3
        let d = FIFO::pop<u64>(&mut f); //4
        assert(a == 1, 1);
        assert(b == 2, 1);
        assert(c == 3, 1);
        assert(d == 4, 1);
        let len = FIFO::len<u64>(& f);
        assert(len == 3, 1);

        FIFO::push<u64>(&mut f, 8);
        FIFO::push<u64>(&mut f, 9);
        FIFO::push<u64>(&mut f, 10);
        let len = FIFO::len<u64>(& f);
        assert(len == 6, 1);

        let e_1 = FIFO::peek<u64>(&mut f);
        assert(*e_1 == 5, 1);
        let len = FIFO::len<u64>(& f);
        assert(len == 6, 1);
        let e_2 = FIFO::pop<u64>(&mut f); //5
        assert(e_2 == 5, 1);
        let len = FIFO::len<u64>(& f);
        assert(len == 5, 1);

        
        let f_1 = FIFO::peek_mut<u64>(&mut f);
        assert(*f_1 == 6, 1);
        *f_1 = 42;
        
        let len = FIFO::len<u64>(& f);
        assert(len == 5, 1);

        let f_2 = FIFO::pop<u64>(&mut f); //42, changed above
        assert(f_2 == 42, 1);
        let len = FIFO::len<u64>(& f);
        assert(len == 4, 1);

        FIFO::push_LIFO<u64>(&mut f, 77);
        let g = FIFO::pop<u64>(&mut f); //77
        assert(g == 77, 1);


        let g = FIFO::pop<u64>(&mut f); //7
        let h = FIFO::pop<u64>(&mut f); //8
        let i = FIFO::pop<u64>(&mut f); //9
        let j = FIFO::pop<u64>(&mut f); //10
        assert(g == 7, 1);
        assert(h == 8, 1);
        assert(i == 9, 1);
        assert(j == 10, 1);  

        let len = FIFO::len<u64>(& f);
        assert(len == 0, 1);      

    }
}
// check: EXECUTED

