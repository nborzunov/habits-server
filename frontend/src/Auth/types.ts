export interface User {
    id: string;
    username?: string;
    email: string;
    fullName?: string;
    bio?: string;
    image?: string;
    emailVerified: boolean;
    active: boolean;
    createdDate: Date;
    updatedDate: Date;
}
