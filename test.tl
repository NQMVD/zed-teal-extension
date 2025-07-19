-- Test Teal file for syntax highlighting

local record Person
    name: string
    age: number
    email: string
end

local interface Drawable
    draw: function(self: Drawable)
end

local enum Color
    "red"
    "green" 
    "blue"
end

global function greet(person: Person): string
    return "Hello, " .. person.name .. "!"
end

local function create_person(name: string, age: number): Person
    return {
        name = name,
        age = age,
        email = name .. "@example.com"
    }
end

-- Test various language constructs
local people: {Person} = {}

for i = 1, 10 do
    local person = create_person("Person " .. i, 20 + i)
    table.insert(people, person)
end

for _, person in ipairs(people) do
    print(greet(person))
end

-- Type annotations and unions
local value: string | number = "hello"
local optional_value: string? = nil

-- Macroexp example
macroexp function debug_print(msg: string)
    if DEBUG then
        print("[DEBUG] " .. msg)
    end
end