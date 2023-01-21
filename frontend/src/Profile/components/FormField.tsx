import {
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
import Icons from '~/common/services/Icons';
import React from 'react';

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
}: FormFieldProps) => {
    console.log(validationError);
    return (
        <Tooltip hasArrow bg='red.600' label={validationError?.message} isOpen={validationError}>
            <FormControl isInvalid={validationError} isRequired={isRequired}>
                <Flex justifyContent={'space-between'}>
                    <FormLabel mr={'6'} lineHeight={'40px'} width={minWidth}>
                        {label}:
                    </FormLabel>
                    <InputGroup>
                        {field !== 'bio' ? (
                            <Input
                                id={field}
                                type={field === 'email' ? 'email' : 'text'}
                                focusBorderColor='purple.500'
                                placeholder={label}
                                variant={'filled'}
                                {...validationProps}
                            />
                        ) : (
                            <Textarea
                                id={field}
                                focusBorderColor='purple.500'
                                placeholder={label}
                                variant={'filled'}
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
                    </InputGroup>
                </Flex>
            </FormControl>
        </Tooltip>
    );
};

FormField.defaultProps = {
    hideResetButton: false,
    isRequired: false,
    minWidth: '140px',
};

export default FormField;