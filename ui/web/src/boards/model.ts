import * as v from 'valibot';

const newBoardSchemaFields = {
    name: v.pipe(v.string(), v.nonEmpty('Name is required')),
    slug: v.optional(v.pipe(
        v.string(),
        v.pipe(
            v.string(),
            v.check((value) => !value || /^[a-z][a-z0-9]*(-[a-z0-9]+)*$/.test(value), 'Slug must start with a letter, contain only lowercase letters, numbers, and hyphens, and not end with a hyphen')
        )
    )),
    description: v.optional(v.string()),
    icon: v.optional(v.string()),
    is_public: v.optional(v.boolean()),
}

// schema for new board item form validation
export const NewBoardSchema = v.object(newBoardSchemaFields)

// type for new board item form values
export type TBoardNew = v.InferOutput<typeof NewBoardSchema>

// schema for existing board item form validation
export const BoardSchema = v.object({
    id: v.string(),
    name: v.pipe(v.string(), v.nonEmpty('Name is required')),
    slug: v.string(), // slug is always present in existing boards
    description: v.optional(v.string()),
    icon: v.optional(v.string()),
    is_public: v.optional(v.boolean()),
})

export type TBoard = v.InferOutput<typeof BoardSchema>

export type TBoardUpdate = Partial<Omit<TBoard, 'id'>>
