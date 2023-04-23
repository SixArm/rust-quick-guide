local chapter_level = 2
local toc_level = 3
local headings = {}
local current_chapter = nil

local function collect_headings(head)
    print("collect headings", head)
        if head.level == 1 then
            if current_chapter then
                local toc = current_chapter.toc
                toc[#toc + 1] = head
            end
        elseif head.level == 2 then
            local id = head.identifier
            current_chapter = {
                chapter = id,
                toc = {}
            }
            headings[id] = current_chapter
        end
    return nil
end

local function collect_headings_2(head)
    print("collect headings", head)
    if head.level == chapter_level then
        local id = head.identifier
        current_chapter = {
            chapter = id,
            toc = {}
        }
        headings[id] = current_chapter
    elseif head.level == toc_level then
        if current_chapter then
            local toc = current_chapter.toc
            toc[#toc + 1] = head
        end
    end
    return nil
end

local function build_toc(heads)
    local toc = {}
    for _, head in ipairs(heads) do
        print("build_toc head", head)
        local entry = {pandoc.Plain {pandoc.Link(head.content:clone(), -- text
        '#' .. head.identifier, -- target
        "", -- empty title
        pandoc.Attr("", -- empty identifier
        {'local-toc-link'} -- class
        ))}}
        toc[#toc + 1] = entry
    end
    return pandoc.Div({pandoc.BulletList(toc)}, pandoc.Attr("", {'local-toc'}))
end

local function insert_toc(head)
    print("insert_toc", head)
    if head.level == chapter_level then
        local id = head.identifier
        if headings[id] then
            local toc = build_toc(headings[id].toc)
            return {head, toc}
        end
    end
    return nil
end

return {{
    Header = collect_headings
}, {
    Header = insert_toc
}}
