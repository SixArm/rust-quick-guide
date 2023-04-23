-- we use pandoc.text to get a UTF-8 aware 'upper' function
local text = pandoc.text

function Header(el)
    if el.level == 2 then
      return el:walk {
        Str = function(el)
            return pandoc.Str(text.upper(el.text))
        end
      }
    end
end

function Link(el)
    return el.content
end

function Note(el)
    return {}
end