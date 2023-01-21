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
import { EditProfileFields } from '~/Profile/types';
import ErrorWrapper from '~/common/components/ErrorWrapper';

interface EditProfileFieldProps {
    field: EditProfileFields;
    label: string;
    initialValue: string;
    value: string;
    validationError: any;
    validationProps: any;
    resetValue: () => void;
}

const EditProfileField = ({
    field,
    label,
    initialValue,
    value,
    validationError,
    validationProps,
    resetValue,
}: EditProfileFieldProps) => {
    return (
        <ErrorWrapper error={validationError}>
            <FormControl isInvalid={validationError}>
                <Flex justifyContent={'space-between'}>
                    <FormLabel mr={'6'} lineHeight={'40px'} width={'140px'}>
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
                        {initialValue !== value && (
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
        </ErrorWrapper>
    );
};

export default EditProfileField