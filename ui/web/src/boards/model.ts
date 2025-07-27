import * as v from 'valibot';

const newBoardSchemaFields = {
    name: v.pipe(v.string(), v.nonEmpty('Name is required')),
    description: v.optional(v.string()),
    icon: v.optional(v.string()),
}

// schema for new board item form validation
export const NewBoardSchema = v.object(newBoardSchemaFields)

// type for new board item form values
export type TBoardNew = v.InferOutput<typeof NewBoardSchema>

// schema for existing board item form validation
export const BoardSchema = v.object({
    id: v.string(),
    ...newBoardSchemaFields
})

export type TBoard = v.InferOutput<typeof BoardSchema>
