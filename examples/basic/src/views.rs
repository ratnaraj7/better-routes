use std::fmt::Display;

use origami_engine::comp;
use serde::Deserialize;
use tailwind_fuse::*;

comp! {
    layout =>
    "<!DOCTYPE html>"!
    html {
        head {
            $head
            script src="https://unpkg.com/htmx.org@2.0.1" integrity="sha384-QWGpdj554B4ETpJJC9z+ZHJcA/i59TyjxEPXiiUgN2WmTyV5OEZWCD6gQhgkdpB/" crossorigin="anonymous" {}
            script src="https://cdn.tailwindcss.com" {}
            style {
                "* {scrollbar-width: thin; scrollbar-color: #9CA3AF #1f2937;} .section { max-width: 1024px; margin: 0 auto; min-width: 375px }"
            }
            script {
                r#"
                    tailwind.config = {
                      theme: {
                        extend: {
                        }
                      }
                    }
                  "#
            }
        }
        body class="min-h-screen bg-slate-900 px-10 py-10 flex" {
            $body
        }
    }
}
pub(crate) use layout;

comp! {
    home =>
    @layout! {
        head {
            title {
                "TODO"
            }
        },
        body {
            div class="m-auto rounded-md bg-slate-800 p-10 space-y-4" {
                h1 class="text-3xl font-bold text-white" {
                    "TODO"
                }
                div class="space-y-8" {
                    form "hx-target"="#rows" "hx-swap"="beforeend" "hx-post"=$create_todo_url class="space-y-4" {
                        input "type"="text" required autocomplete="off" name="name" placeholder="Name" class="w-full px-4 py-2 rounded-md bg-slate-700 text-white focus:outline-none";
                        button "type"="submit" class="w-full px-4 py-2 rounded-md bg-slate-300 text-slate-800 focus:outline-none" {
                            "Add"
                        }
                    }
                    table class="w-[60rem] text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400" {
                        @row_head!{};
                        tbody id="rows" {
                            for row in $rows.iter(); {
                                @row!{
                                    id { @(format!("row-{}", row.id).as_str()) },
                                    status { @(StatusClass { status: row.status }.to_class().as_str()) },
                                    name { *row.name.as_str(); },
                                    delete_todo_url { @(row.delete_todo_url.as_str()) },
                                    target { @(format!("#row-{}", row.id).as_str()) },
                                    onclick { @(format!("show_dialog('#row-{}', '{}', '{}', '{}')", row.id, row.update_todo_url, row.name, row.status).as_str()) }
                                };
                            }
                        }
                    }
                    dialog class="h-screen w-full bg-black/40" {
                        div class="w-full h-full flex" {
                            form id="dialog-form" class="m-auto w-[30rem] p-4 bg-slate-800 rounded-md space-y-4" {
                                input id="dialog-input" "type"="text" required autocomplete="off" name="name" placeholder="Name" class="w-full px-4 py-2 rounded-md bg-slate-700 text-white focus:outline-none";
                                select id="dialog-select" class="w-full px-4 py-2 rounded-md bg-slate-700 text-white focus:outline-none" name="status" {
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
                                document.getElementById("dialog-select").querySelector(`option[value="${status}"]`).selected = true;
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
                        "#
                    }
                }
            }
        }
    };
}
pub(crate) use home;

comp! {
    row_head =>
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
pub(crate) use row_head;

#[derive(TwClass)]
#[tw(class = "px-6 py-4 font-medium text-gray-900")]
pub struct StatusClass {
    pub status: Status,
}

#[derive(Deserialize, TwVariant)]
pub enum Status {
    #[tw(class = "text-green-500")]
    Todo,
    #[tw(default, class = "text-red-500 line-through")]
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::Done => write!(f, "Done"),
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

comp! {
    row =>
    tr id=$id class="bg-white border-b dark:bg-gray-800 dark:border-gray-700" {
        th scope="row"
            class=$status {
                $name
        }
        td class="px-6 py-4 space-x-2" {
            a class="decoration-wavy underline text-red-500 cursor-pointer" "hx-delete"=$delete_todo_url "hx-target"=$target "hx-swap"="outerHTML" {
                "Delete"
            }
            a class="decoration-wavy underline text-slate-500 cursor-pointer" onclick=$onclick {
                "Update"
            }
        }
    }
}
pub(crate) use row;
