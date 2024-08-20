use origami_engine::{og, Layout, Origami};

use crate::Status;

struct Main;
og! {
    layout MainLayout,
    Main =>
    "<!DOCTYPE html>"!
    html {
        head {
            block head;
            script src="https://unpkg.com/htmx.org@2.0.1" integrity="sha384-QWGpdj554B4ETpJJC9z+ZHJcA/i59TyjxEPXiiUgN2WmTyV5OEZWCD6gQhgkdpB/" crossorigin="anonymous" {}
            script src="https://cdn.tailwindcss.com" {}
            style {
                "* {scrollbar-width: thin; scrollbar-color: #9CA3AF #1f2937;} .section { max-width: 1024px; margin: 0 auto; min-width: 375px }"!
            }
            script {
                r#"
                    tailwind.config = {
                      theme: {
                        extend: {
                        }
                      }
                    }
                  "#!
            }
        }
        body class="h-screen bg-slate-900 overflow-hidden px-10 py-10 flex" {
            block body;
        }
    }
}

pub struct Home {
    pub rows: Vec<Row>,
    pub create_todo_url: String,
}

og! {
    @Main{} MainLayout,
    Home =>
    extend head {
        title {
            "TODO"
        }
    }
    extend body {
        div class="m-auto rounded-md bg-slate-800 p-10 space-y-4" {
            h1 class="text-3xl font-bold text-white" {
                "TODO"
            }
            div class="space-y-8" {
                form "hx-target"="#rows" "hx-swap"="beforeend" "hx-post"=(self.create_todo_url.as_str()) class="space-y-4" {
                    input "type"="text" required autocomplete="off" name="name" placeholder="Name" class="w-full px-4 py-2 rounded-md bg-slate-700 text-white focus:outline-none";
                    button "type"="submit" class="w-full px-4 py-2 rounded-md bg-slate-300 text-slate-800 focus:outline-none" {
                        "Add"
                    }
                }
                table class="w-[60rem] text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400" {
                    @RowHead{};
                    tbody id="rows" {
                        @(..self.rows.iter());
                    }
                }
                dialog class="h-screen w-full bg-black/40" {
                    div class="w-full h-full flex" {
                        form id="dialog-form" class="m-auto w-[30rem] p-4 bg-slate-800 rounded-md space-y-4" {
                            input id="dialog-input" "type"="text" required autocomplete="off" name="name" placeholder="Name" class="w-full px-4 py-2 rounded-md bg-slate-700 text-white focus:outline-none";
                            select class="w-full px-4 py-2 rounded-md bg-slate-700 text-white focus:outline-none" name="status" {
                                option "value"="Todo" {
                                    "Todo"
                                }
                                option "value"="Done" {
                                    "Done"
                                }
                            }
                            button "type"="submit" class="w-full px-4 py-2 rounded-md bg-slate-300 text-slate-800 focus:outline-none" {
                                "Update"
                            }
                            button "type"="button" onclick="hide_dialog()" class="w-full px-4 py-2 rounded-md bg-red-500 text-slate-800 focus:outline-none" {
                                "Close"
                            }
                        }
                    }
                }
                script {
                    r#"
                        function show_dialog(id, update_url, name, status) {
                            document.querySelector("dialog").showModal();
                            document.getElementById("dialog-input").value = name;
                            document.getElementById("dialog-form").onsubmit = function(evt) {
                                evt.preventDefault();
                                const values = htmx.values(htmx.find('#dialog-form'));
                                htmx.ajax('PUT', update_url, {
                                    target: id,
                                    swap: "outerHTML",
                                    values
                                });
                                hide_dialog();
                            };
                        }
                        
                        function hide_dialog() {
                            document.querySelector("dialog").close();
                        }
                    "#!

                }
            }
        }
    }
}

pub struct RowHead;

og! {
    RowHead =>
    thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400" {
        tr {
            th scope="col" class="px-6 py-3 w-10/12" {
                "Name"
            }
            th class="px-6 py-3" {
                "Action"
            }
        }
    }
}

pub struct Row {
    pub id: usize,
    pub name: String,
    pub status: Status,
    pub delete_todo_url: String,
    pub update_todo_url: String,
}

og! {
    Row =>
    tr id=(format!("row-{}", self.id).as_str()) class="bg-white border-b dark:bg-gray-800 dark:border-gray-700" {
        th scope="row"
            class=(
                match self.status {
                    Status::Done => "px-6 py-4 font-medium text-gray-900 whitespace-nowrap text-red-500 line-through flex",
                    Status::Todo => "px-6 py-4 font-medium text-gray-900 whitespace-nowrap text-green-500 flex",
                }
            ) {
            (self.name.as_str());
        }
        td class="px-6 py-4 space-x-2" {
            a class="decoration-wavy underline text-red-500 cursor-pointer" "hx-delete"=(self.delete_todo_url.as_str()) "hx-target"=(format!("#row-{}", self.id).as_str()) "hx-swap"="outerHTML" {
                "Delete"
            }
            a class="decoration-wavy underline text-slate-500 cursor-pointer" onclick=(format!("show_dialog('#row-{}', '{}', '{}', '{}')", self.id, self.update_todo_url, self.name, self.status).as_str()) {
                "Update"
            }
        }
    }
}
