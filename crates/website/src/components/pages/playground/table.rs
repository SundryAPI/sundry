use crate::components::atoms::table::{Row, RowType, Table};
use crate::components::atoms::tag::{Tag, TagVariant};
use leptos::prelude::*;

#[component]
pub fn TablePlayground() -> impl IntoView {
    view! {
        <div>
            <Table>
                <Row _type=RowType::Header>
                    <p>Header 1</p>
                    <p>Header 2</p>
                    <p>Header 3</p>
                </Row>
                <Row>
                    <p>Row 1</p>
                    <p>Row 1</p>
                    <Tag variant=TagVariant::Active />
                </Row>
                <Row>
                    <p>Row 2</p>
                    <p>Row 2</p>
                    <Tag variant=TagVariant::Invited />
                </Row>
            </Table>
        </div>
    }
}
