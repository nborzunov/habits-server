import { Tooltip } from '@chakra-ui/react';
import React from 'react';

const ErrorWrapper = ({ error, children }: { error: any; children: any }) => {
    return error ? (
        <Tooltip hasArrow bg='red.600' label={error.message} isOpen>
            {children}
        </Tooltip>
    ) : (
        children
    );
};

export default ErrorWrapper;
