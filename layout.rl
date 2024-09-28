Layout {
    w: 33%,
    h: 33%,
    DynamicRow {
        source: "joe",
        Button {
            text: "mama: " + {mama},
            callback: "print chud"
        }
    }
}

layout (
    w: 33%, 
    h: 33%,
    drow (
        "joe",
        button ("mama: " + mama, "print chud")
    )
)
