import {
    Button,
    Flex,
    FormControl,
    FormLabel,
    Icon,
    IconButton,
    Input,
    InputGroup,
    InputRightElement,
    Textarea,
    Tooltip,
} from '@chakra-ui/react';
import Icons from '~/common/helpers/Icons';
import React, { useState } from 'react';
import ErrorWrapper from '~/common/components/ErrorWrapper';

interface FormFieldProps {
    field: string;
    label: string;
    initialValue?: string;
    value?: string;
    validationError: any;
    validationProps: any;
    hideResetButton: boolean;
    isRequired: boolean;
    minWidth: string;
    resetValue?: () => void;
    direction: 'row' | 'column';
    variant: 'outline' | 'filled';
    showPasswordIcon: boolean;
}

const FormField = ({
    field,
    label,
    initialValue,
    value,
    hideResetButton,
    minWidth,
    isRequired,
    validationError,
    validationProps,
    resetValue,
    direction,
    variant,
    showPasswordIcon,
}: FormFieldProps) => {
    const [showPassword, setShowPassword] = useState(false);

    return (
        <ErrorWrapper error={validationError}>
            <FormControl isInvalid={validationError} isRequired={isRequired}>
                <Flex flexDirection={direction} justifyContent={'space-between'}>
                    <FormLabel mr={'6'} mb={0} lineHeight={'40px'} width={minWidth}>
                        {label}
                        {direction === 'row' && ':'}
                    </FormLabel>
                    <InputGroup>
                        {field !== 'bio' ? (
                            <Input
                                id={field}
                                type={field === 'email' ? 'email' : 'text'}
                                focusBorderColor='purple.500'
                                placeholder={label}
                                variant={variant}
                                {...validationProps}
                            />
                        ) : (
                            <Textarea
                                id={field}
                                focusBorderColor='purple.500'
                                placeholder={label}
                                variant={variant}
                                {...validationProps}
                            />
                        )}
                        {initialValue !== value && !hideResetButton && (
                            <InputRightElement>
                                <Tooltip label={'Reset to previous'}>
                                    <IconButton
                                        icon={<Icon as={Icons.Reset} />}
                                        aria-label={'reset'}
                                        size='sm'
                                        variant={'unstyled'}
                                        _hover={{ color: 'purple.500' }}
                                        onClick={resetValue}
                                    />
                                </Tooltip>
                            </InputRightElement>
                        )}
                        {showPasswordIcon && (
                            <InputRightElement h={'full'}>
                                <Button
                                    variant={'ghost'}
                                    onClick={() => setShowPassword((showPassword) => !showPassword)}
                                >
                                    {showPassword ? (
                                        <Icon as={Icons.Show} />
                                    ) : (
                                        <Icon as={Icons.Hide} />
                                    )}
                                </Button>
                            </InputRightElement>
                        )}
                    </InputGroup>
                </Flex>
            </FormControl>
        </ErrorWrapper>
    );
};

FormField.defaultProps = {
    hideResetButton: false,
    isRequired: false,
    minWidth: '140px',
    direction: 'row',
    variant: 'filled',
    showPasswordIcon: false,
};

export default FormField;

type FieldConfig<T> = {
    field: T;
    label: string;
    validationProps: any;
};
export type FieldsConfig<T> = Array<FieldConfig<T>>;

export type FieldsConfigObj<T extends string> = {
    [key in T]: FieldConfig<T>;
};
