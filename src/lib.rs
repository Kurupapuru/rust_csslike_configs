pub(crate) mod tokenization;

#[cfg(test)]
mod tests;

mod selectors {
    pub struct ObjectCssDesc {
        pub tags: Vec<Tag>
    }

    pub struct Tag(String);

    pub trait Selector {
        fn check(&self, object_desc: &ObjectCssDesc) -> bool;
    }

    struct SelectorHasTag(String);
    impl Selector for SelectorHasTag {
        fn check(&self, object_desc: &ObjectCssDesc) -> bool {
            todo!()
        }
    }

    struct SelectorAny(Vec<Box<dyn Selector>>);
    impl Selector for SelectorAny{
        fn check(&self, object_desc: &ObjectCssDesc) -> bool {
            self.0.iter().any(|x| x.check(object_desc))
        }
    }

    struct SelectorAll(Vec<Box<dyn Selector>>);
    impl Selector for SelectorAll {
        fn check(&self, object_desc: &ObjectCssDesc) -> bool {
            self.0.iter().all(|x| x.check(object_desc))
        }
    }
}