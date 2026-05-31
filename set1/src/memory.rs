

pub fn memory_traits_pointers() {
    // traits are behaviour contracts
    trait Speak {
        fn speak(&self) -> String;
    }

    // type
    struct Dog;

    // implement trait for a type
    impl Speak for Dog {
        fn speak(&self) -> String {
            "wooof".to_string()
        }
    }
    
    let input: String = "ASD".to_string();

    // Rust Memory Management
    // every valeus has owner, moving value transfers owner, owner drop = cleanup
    // Borrowing & without taking ownership
    // many immutable borrows &T
    // or single mut borrow &mut T

    
    // stack automatic lifetime, fast, fixed sized
    // heap dynamic size allocator involved
    

    // borrowed byte slice. Fat pointer: pointer to first byte + length.
    // Fat pointer : if pointer pointing to dynamically sized type.
    // no allocation cost.
    let byteVersion : &[u8] = input.as_bytes();


    // owned buffer in heap.
    let byteVecVersion : Vec<u8> = input.as_bytes().to_vec();

    let refToVect : &Vec<u8> = &byteVecVersion;


    // [u8] is not sized. i.e. sized trait is not implemented.
    // because in compile time we don't know the size of what [u8] is.
    

    // so following not possible. all local variables must have it known size in compile time.
    // let bunch_of_bytes : [u8] = *input.as_bytes();

    // but it is possible for Vec<>. because essentially it is a struct like structure
    // it has a pointer to start, length and caapcity. So Vec<u8> is esseantially:
    struct Veclike {
        ptr: *mut u8,
        size: usize,
        cap: usize,
    }
    // so so the header szie for the Vec<> is always known. Vec<> is sized. Again it is a reference to the data.

    // Smart Pointers

    // 1- Box<T> = own a value on the heap with single owner.
    // Box handle itself lives on stack generally. 
    // T lives on heap.
    // when Box goes out of scope Rust automatically Drop, frees heap memory.
    // can be used for: recursion, Trait object for runtime polymorphism Box <dyn Trait>
    // with Box<T> we can put large object to heap and transfer ownership of heap values cleanly.
    

    // 2- Rc<T>: single threaded, shared ownership

    // 3- Arc<T>: thread safe atomic, shared ownership

    // 4- Refcell<T>: single threaded interior mutablity, runtime borrowchecks

    // 5- Mutex<T> / RwLock<T>: syncronized mutable checks across thread


    // Rust's static polymorphism Generics: let you write one fucntion/type for many concrete types with compile-time checking.
    // fn identity<T>(x: T) -> T {
    // }
    // T is placeholder. 
    // when function is used,  Rust creates concrete versions monomorphization.
    // identity_i32, identity_string etc. 
   
    // generic trait, static dispatch
    fn make_sound_generic_moved<T: Speak>(x: T) {
        print!("{}", x.speak()) // auto-borrows as &x for speak(&self)
    }
    fn make_sound_generic_borrowed<T: Speak>(x: &T) { // avoids moving x to the function. match common trait definition
        print!("{}", x.speak())   
    }


    // impl Trait:
    // can be used in fuction arguments, it is effectively generic/static dispatch.
    fn print_len(x: impl AsRef<[u8]>) {
        println!("{}", x.as_ref().len());
    } 
    // equals to:
    fn print_len2<T: AsRef<[u8]>>(x : T) {
        println!("{}", x.as_ref().len());
    }


    // Rust's runtime polymorphism: Box<dyn Trait>. when you need a heterogeneous collection
    // Trait says what methods are available, 
    // dyn Trait says the concrete type is erased at compile time
    // dyn Trait is a DST, dynamically Sized Type.
    // Box <dyn Trait> gives it a known, fixed-size owner, i..e heap pointer. so you can own and pass it.

    // since dyn Trait is a DST, it's size is not known in compile time, so it can't live by value, 
    // it must be put behind a pointer-like type: 
    // & (single), Box<> (single), Rc<> (shared), Arc<> (shared) 

    // is it a fat pointer? Box<dyn Trait>, Yes:
    // there is pointer to data
    // there is pointer to vtable which contains methdo pointers. 

    // Dispatch: static (generics/impl Trait) vs dyanmic (dyn Trait)
    // static is fast, resolved at compile time, potentially larger binaries as multiple of monomorphized copies
    // dynamic is slower, resolved at runtime, one indirect call.


    // vtable is the mechanism to make dyn Trait method calls work at runtime. 
    // traits objects liek &dyn Trait or Box<dyn Trait> concrete type is not known on compile time, but on runtime.
    // vtable is a metadata that stores: "which concrete type is this?" and "which function impl should be called?"


    // object-safe traits can become dyn Trait.
    // for a trait to be usable as dyn Trait, every callable method mmust have single, type-erased calling shape.
    // this is object safety. Why some methods break it?
    // If a method needs concrete Self, in cannot be called through dyn Trait because concrete type is hidden
    // 1) returning self breaks object safety
    trait Clonelike {
        fn make(&self) -> Self; // not object-safe for dyn calls
    }
    // 2) generic methods breaj object safety
    trait Bad {
        fn parse<T>(&self, s:&str) -> T; // vtable would need infinite version of all possible Ts.
    }
    // What is allowed?
    // do not have generic params, do not return self
    // use Self in ways requiring concrete layout.
    trait Speak2 {
        fn Speak(&self); // does not return self.
        fn rename(&mut self, s: &str);
    }

   


    // dynamic Trait, trait is used as object
    fn make_sound_dyn(x: Box<dyn Speak>){
        println!("{}", x.speak());
    }
    fn make_sound_dyn2(x: &dyn Speak){
        println!("{}", x.speak());
    }



    // How a dynamic call works?
    fn render(x : &dyn Speak) {
        x.speak();
    }


  
}






/* 
devam etmeden once sana kendi duzenledigim halini atiyorum tezin, 4. bolumun en son hali bunda var, benim yazi stilime dikkat et. 

Hatirlatma: simdi 4 uncu bolumu bayagi bir guncelledik. 5. bolumun de guncellenmesi gerek yerler olacak muhakkak. 5, bolumu tekrar bir oku, 4.bolumu de bir oku, sonra 5. bolumude gerekli guncellemeleri yap. ekstra cikarimlar varsa yap, bulgularda degisme varsa yap.

bir de 5. bolumu yazarken 4 te sunulmus her seyin degerlenidrildiginden emin ol. unutma olmasin. 
*/