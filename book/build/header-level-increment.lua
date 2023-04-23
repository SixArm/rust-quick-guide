-- equivalent to: --shift-heading-level-by=1

function Header(el)
    el.level = el.level + 1
    return el
end
