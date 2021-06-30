//! account: alice, 1000000, 0, validator

//! new-transaction
module M {
    use 0x1::Event::{EventHandle, emit_event, new_event_handle, destroy_handle};
    use 0x1::Signer::address_of;

    resource struct MyEvent<T: copyable> {
        e: EventHandle<T>
    }

    fun maybe_init_event<T: copyable>(s: signer) {
        if (exists<MyEvent<T>>(address_of(s))) return;

        move_to(s, MyEvent<T> { e: new_event_handle<T>(s)})
    }

    public fun emit(s: signer) acquires MyEvent {
        maybe_init_event<bool>(s);

        emit_event(&mut borrow_global_mut<MyEvent<bool>>(address_of(s)).e, true);

        let MyEvent<bool> { e } = move_from<MyEvent<bool>>(address_of(s));
        destroy_handle(e);
    }
}

//! new-transaction
//! sender: alice
script {
    use {{default}}::M;

    fun main(s: signer) {
        M::emit(s);
    }
}
// check: EXECUTED
// check: ContractEvent
// check: Bool
