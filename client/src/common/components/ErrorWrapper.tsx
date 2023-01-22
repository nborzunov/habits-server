import { Tooltip } from '@chakra-ui/react';

const ErrorWrapper = ({ error, children }: any) => {
    return (
        <Tooltip hasArrow bg='red.600' label={error?.message} isOpen={error}>
            {children}
        </Tooltip>
    );
};

export default ErrorWrapper;
