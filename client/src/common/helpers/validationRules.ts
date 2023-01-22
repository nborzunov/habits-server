const validationRules = {
    password: () => ({
        required: { value: true, message: 'Current password is required' },
        minLength: { value: 8, message: 'Password must be at least 8 characters' },
    }),
    newPassword: () => ({
        required: { value: true, message: 'Password is required' },
        minLength: { value: 8, message: 'Password must be at least 8 characters' },
        pattern: {
            value: /^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$/,
            message: 'Password must contain at least one letter and one number',
        },
    }),
    newPasswordConfirm: (password: string) => ({
        required: { value: true, message: 'Confirm new password is required' },
        validate: (value: string) => {
            if (value !== password) {
                return 'Passwords do not match';
            }
        },
    }),
    text: (minLength: number) => ({
        required: 'This is required',
        minLength: { value: minLength, message: `Minimum length should be ${minLength}` },
    }),
    longText: () => ({
        maxLength: { value: 250, message: 'Maximum length should be 250' },
    }),
    email: () => ({
        required: 'This is required',
        pattern: {
            value: /\S+@\S+\.\S+/,
            message: 'Invalid email address',
        },
    }),
};

export default validationRules;
