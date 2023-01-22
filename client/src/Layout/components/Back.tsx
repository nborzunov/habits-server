import { Icon, IconButton, Tooltip } from '@chakra-ui/react';
import Icons from '~/common/helpers/Icons';
import React from 'react';

const Back = () => {
    return (
        <Tooltip label={'Back'}>
            <IconButton
                icon={<Icon as={Icons.Back} />}
                fontSize={'xl'}
                aria-label={'back'}
                size='lg'
                colorScheme={'purple'}
                variant={'outline'}
            />
        </Tooltip>
    );
};

export default Back;
