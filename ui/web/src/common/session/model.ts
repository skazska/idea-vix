import * as v from 'valibot';

// schema for sign in form validation
export const SignInSchema = v.object({
    address: v.pipe(v.string(), v.email('Please enter a valid email address')),
});

// type for sign in form values
export type TSignInRequest = v.InferOutput<typeof SignInSchema>;

// schema for verify code form validation
export const VerifyCodeSchema = v.object({
    address: v.pipe(v.string(), v.email()),
    code: v.pipe(v.string(), v.nonEmpty('Verification code is required')),
});

// type for verify code form values
export type TVerifyCodeRequest = v.InferOutput<typeof VerifyCodeSchema>;

// type for session data
export type TSessionData = {
    address: string;
    sent_at: number;
    expires_at: number;
    token?: string; // Now optional as it will be in the cookie
};
