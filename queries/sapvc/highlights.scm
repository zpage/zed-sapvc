; Tree-sitter highlight queries for the SAP VC/AVC dependency language.
; Consumed by Zed (primary) and by `tree-sitter highlight` for offline testing.
; Node names come from grammar.js (VALIDATED against 17,988 lines of production
; source, 2026-08-03).

(comment) @comment
(string) @string
(number) @number

; $SELF / $ROOT / $PARENT
(object_var) @variable.builtin

; bare characteristic/object references: ET1.CHAR, TXT_DESIGN_CODE, M.TYP_EM
(reference (identifier) @variable)

; control keywords
"IF" @keyword.control
"is" @keyword.operator
"where" @keyword.operator

; logical / relational operators
"AND" @keyword.operator
"OR" @keyword.operator
"NOT" @keyword.operator
"IN" @keyword.operator
"SPECIFIED" @keyword.operator

; constraint sections
"OBJECTS" @keyword
"CONDITION" @keyword
"RESTRICTIONS" @keyword
"INFERENCES" @keyword

; object declaration
"IS_A" @keyword

; function-style predicates
"PART_OF" @function.call
"SUB_PART_OF" @function.call
"TYPE_OF" @function.call

; keyword calls (variant table / custom function invocations)
"table" @keyword
"function" @keyword
"pfunction" @keyword
"FUNCTION" @keyword

; system procedure verbs
"$SET_DEFAULT" @function.builtin
"$DEL_DEFAULT" @function.builtin
"$SET_PRICING_FACTOR" @function.builtin
"$COUNT_PARTS" @function.builtin
"$SUM_PARTS" @function.builtin

; operators
"=" @operator
"?=" @operator
"<>" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"||" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
