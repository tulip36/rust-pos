use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "min-h-screen bg-gray-50",
            header {
                class: "bg-white shadow-sm p-4",
                div {
                    class: "max-w-4xl mx-auto",
                    h1 {
                        class: "text-2xl font-bold",
                        "🏪 rust-pos - 中餐馆点餐系统"
                    }
                }
            }
            main {
                class: "max-w-4xl mx-auto p-4",
                div {
                    class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                    
                    // 顾客端
                    div {
                        class: "bg-white p-6 rounded-lg shadow",
                        h2 { class: "font-semibold mb-2", "👤 顾客端" }
                        p { class: "text-sm text-gray-500", "点餐、支付" }
                    }
                    
                    // 厨房端
                    div {
                        class: "bg-white p-6 rounded-lg shadow",
                        h2 { class: "font-semibold mb-2", "👨‍🍳 厨房端" }
                        p { class: "text-sm text-gray-500", "KDS 厨房显示" }
                    }
                    
                    // 服务员端
                    div {
                        class: "bg-white p-6 rounded-lg shadow",
                        h2 { class: "font-semibold mb-2", "👨‍💼 服务员" }
                        p { class: "text-sm text-gray-500", "开台、点餐" }
                    }
                    
                    // 管理端
                    div {
                        class: "bg-white p-6 rounded-lg shadow",
                        h2 { class: "font-semibold mb-2", "⚙️ 管理端" }
                        p { class: "text-sm text-gray-500", "菜品、订单" }
                    }
                }
                
                div {
                    class: "mt-8 p-4 bg-blue-50 rounded-lg",
                    h3 { class: "font-semibold mb-2", "API 状态" }
                    p { class: "text-sm", "服务器运行中: http://localhost:3000" }
                }
            }
        }
    })
}
