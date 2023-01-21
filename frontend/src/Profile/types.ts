export interface User {
    id: string;
    username?: string;
    email: string;
    name?: string;
    surname?: string;
    bio?: string;
    image?: string;
    emailVerified: boolean;
    active: boolean;
    createdDate: Date;
    updatedDate: Date;
}

export interface ProfileData {
    name: string;
    surname: string;
    username: string;
    email: string;
    password: string;
}

export type ProfileDataFields = 'name' | 'surname' | 'username' | 'email' | 'bio';

export type FieldsConfig<T = ProfileDataFields> = Array<{
    field: T;
    label: string;
    validationProps: any;
}>;