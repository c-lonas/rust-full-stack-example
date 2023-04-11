use yew::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <section class="dashboard">
            <div class="dashboard-card">
                <h3>{ "Total Income" }</h3>
                <p>{ "Placeholder for total income value" }</p>
            </div>
            <div class="dashboard-card">
                <h3>{ "Total Expenses" }</h3>
                <p>{ "Placeholder for total expenses value" }</p>
            </div>
            <div class="dashboard-card">
                <h3>{ "Savings" }</h3>
                <p>{ "Placeholder for savings value" }</p>
            </div>
            <div class="dashboard-card">
                <h3>{ "Budget" }</h3>
                <p>{ "Placeholder for budget value" }</p>
            </div>
            
        </section>
    }
}
