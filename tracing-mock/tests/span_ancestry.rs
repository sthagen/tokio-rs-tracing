//! Tests assertions for the parent made on [`ExpectedSpan`].
//!
//! The tests in this module completely cover the positive and negative cases
//! when expecting that a span is a contextual or explicit root or expecting
//! that a span has a specific contextual or explicit parent.
//!
//! [`ExpectedSpan`]: crate::span::ExpectedSpan
//!
use tracing::{collect::with_default, Level};
use tracing_mock::{collector, expect};

#[test]
fn contextual_parent() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_contextual_parent("contextual parent"));

    let (collector, handle) = collector::mock()
        .enter(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let _guard = tracing::info_span!("contextual parent").entered();
        tracing::info_span!("span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(
    expected = "to have a contextual parent span named `contextual parent`,\n\
    [contextual_parent_wrong_name] but got one named `another parent` instead."
)]
fn contextual_parent_wrong_name() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_contextual_parent("contextual parent"));

    let (collector, handle) = collector::mock()
        .enter(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let _guard = tracing::info_span!("another parent").entered();
        tracing::info_span!("span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to have a contextual parent span a span with Id `1`,\n\
    [contextual_parent_wrong_id] but got one with Id `2` instead")]
fn contextual_parent_wrong_id() {
    let id = expect::id();
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_contextual_parent(&id));

    let (collector, handle) = collector::mock()
        .new_span(&id)
        .new_span(expect::span())
        .enter(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let _span = tracing::info_span!("contextual parent");
        let _guard = tracing::info_span!("another parent").entered();
        tracing::info_span!("span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(
    expected = "to have a contextual parent span at level `Level(Info)`,\n\
    [contextual_parent_wrong_level] but got one at level `Level(Debug)` instead."
)]
fn contextual_parent_wrong_level() {
    let parent = expect::span().at_level(Level::INFO);
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_contextual_parent(parent));

    let (collector, handle) = collector::mock()
        .enter(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let _guard = tracing::debug_span!("contextual parent").entered();
        tracing::info_span!("span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to have a contextual parent span, but it is actually a \
    contextual root")]
fn expect_contextual_parent_actual_contextual_root() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_contextual_parent("contextual parent"));

    let (collector, handle) = collector::mock().new_span(span).run_with_handle();

    with_default(collector, || {
        tracing::info_span!("span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to have a contextual parent span, but it actually has an \
    explicit parent span")]
fn expect_contextual_parent_actual_explicit_parent() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_contextual_parent("contextual parent"));

    let (collector, handle) = collector::mock()
        .new_span(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let span = tracing::info_span!("explicit parent");
        tracing::info_span!(parent: span.id(), "span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to have a contextual parent span, but it is actually an \
    explicit root")]
fn expect_contextual_parent_actual_explicit_root() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_contextual_parent("contextual parent"));

    let (collector, handle) = collector::mock()
        .enter(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let _guard = tracing::info_span!("contextual parent").entered();
        tracing::info_span!(parent: None, "span");
    });

    handle.assert_finished();
}

#[test]
fn contextual_root() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::is_contextual_root());

    let (collector, handle) = collector::mock().new_span(span).run_with_handle();

    with_default(collector, || {
        tracing::info_span!("span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to be a contextual root, but it actually has a contextual parent span")]
fn expect_contextual_root_actual_contextual_parent() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::is_contextual_root());

    let (collector, handle) = collector::mock()
        .enter(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let _guard = tracing::info_span!("contextual parent").entered();
        tracing::info_span!("span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to be a contextual root, but it actually has an explicit parent span")]
fn expect_contextual_root_actual_explicit_parent() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::is_contextual_root());

    let (collector, handle) = collector::mock()
        .new_span(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let span = tracing::info_span!("explicit parent");
        tracing::info_span!(parent: span.id(), "span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to be a contextual root, but it is actually an explicit root")]
fn expect_contextual_root_actual_explicit_root() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::is_contextual_root());

    let (collector, handle) = collector::mock()
        .enter(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let _guard = tracing::info_span!("contextual parent").entered();
        tracing::info_span!(parent: None, "span");
    });

    handle.assert_finished();
}

#[test]
fn explicit_parent() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_explicit_parent("explicit parent"));

    let (collector, handle) = collector::mock()
        .new_span(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let span = tracing::info_span!("explicit parent");
        tracing::info_span!(parent: span.id(), "span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(
    expected = "to have an explicit parent span named `explicit parent`,\n\
    [explicit_parent_wrong_name] but got one named `another parent` instead."
)]
fn explicit_parent_wrong_name() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_explicit_parent("explicit parent"));

    let (collector, handle) = collector::mock()
        .new_span(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let span = tracing::info_span!("another parent");
        tracing::info_span!(parent: span.id(), "span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to have an explicit parent span a span with Id `1`,\n\
    [explicit_parent_wrong_id] but got one with Id `2` instead")]
fn explicit_parent_wrong_id() {
    let id = expect::id();
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_explicit_parent(&id));

    let (collector, handle) = collector::mock()
        .new_span(&id)
        .new_span(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let _span = tracing::info_span!("explicit parent");
        let another_span = tracing::info_span!("another parent");
        tracing::info_span!(parent: another_span.id(), "span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to have an explicit parent span at level `Level(Info)`,\n\
    [explicit_parent_wrong_level] but got one at level `Level(Debug)` instead.")]
fn explicit_parent_wrong_level() {
    let parent = expect::span().at_level(Level::INFO);
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_explicit_parent(parent));

    let (collector, handle) = collector::mock()
        .new_span(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let span = tracing::debug_span!("explicit parent");
        tracing::info_span!(parent: span.id(), "span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to have an explicit parent span, but it actually has a \
    contextual parent span")]
fn expect_explicit_parent_actual_contextual_parent() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_explicit_parent("explicit parent"));

    let (collector, handle) = collector::mock()
        .enter(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let _guard = tracing::info_span!("contextual parent").entered();
        tracing::info_span!("span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to have an explicit parent span, but it is actually a \
    contextual root")]
fn expect_explicit_parent_actual_contextual_root() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_explicit_parent("explicit parent"));

    let (collector, handle) = collector::mock().new_span(span).run_with_handle();

    with_default(collector, || {
        tracing::info_span!("span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to have an explicit parent span, but it is actually an \
    explicit root")]
fn expect_explicit_parent_actual_explicit_root() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::has_explicit_parent("explicit parent"));

    let (collector, handle) = collector::mock()
        .enter(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let _guard = tracing::info_span!("contextual parent").entered();
        tracing::info_span!(parent: None, "span");
    });

    handle.assert_finished();
}

#[test]
fn explicit_root() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::is_explicit_root());

    let (collector, handle) = collector::mock()
        .new_span(expect::span())
        .enter(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let _guard = tracing::info_span!("contextual parent").entered();
        tracing::info_span!(parent: None, "span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to be an explicit root, but it actually has a contextual parent span")]
fn expect_explicit_root_actual_contextual_parent() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::is_explicit_root());

    let (collector, handle) = collector::mock()
        .enter(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let _guard = tracing::info_span!("contextual parent").entered();
        tracing::info_span!("span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to be an explicit root, but it is actually a contextual root")]
fn expect_explicit_root_actual_contextual_root() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::is_explicit_root());

    let (collector, handle) = collector::mock().new_span(span).run_with_handle();

    with_default(collector, || {
        tracing::info_span!("span");
    });

    handle.assert_finished();
}

#[test]
#[should_panic(expected = "to be an explicit root, but it actually has an explicit parent span")]
fn expect_explicit_root_actual_explicit_parent() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::is_explicit_root());

    let (collector, handle) = collector::mock()
        .new_span(expect::span())
        .new_span(span)
        .run_with_handle();

    with_default(collector, || {
        let span = tracing::info_span!("explicit parent");
        tracing::info_span!(parent: span.id(), "span");
    });

    handle.assert_finished();
}

#[test]
fn explicit_and_contextual_root_is_explicit() {
    let span = expect::span()
        .named("span")
        .with_ancestry(expect::is_explicit_root());

    let (collector, handle) = collector::mock().new_span(span).run_with_handle();

    with_default(collector, || {
        tracing::info_span!(parent: None, "span");
    });

    handle.assert_finished();
}