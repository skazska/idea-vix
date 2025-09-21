import * as v from 'valibot';

const newPackageSchemaFields = {
    name: v.pipe(v.string(), v.nonEmpty('Name is required')),
    slug: v.optional(v.pipe(v.string(), v.regex(/^[a-z][a-z0-9]*(-[a-z0-9]+)*$/, 'Slug must start with a letter, contain only lowercase letters, numbers, and hyphens, and not end with a hyphen'))),
    description: v.optional(v.string()),
    icon: v.optional(v.string()),
    is_public: v.optional(v.boolean()),
}

// schema for new package item form validation
export const NewPackageSchema =  v.object(newPackageSchemaFields)

// type for new package item form values
export type TPackageNew = v.InferOutput<typeof NewPackageSchema>

// schema for existing package item form validation
export const PackageSchema = v.object({
    id: v.string(),
    slug: v.string(),
    name: v.pipe(v.string(), v.nonEmpty('Name is required')),
    description: v.optional(v.string()),
    icon: v.optional(v.string()),
    is_public: v.optional(v.boolean()),
})

export type TPackage = v.InferOutput<typeof PackageSchema>
