import { Icon, IconButton, Tooltip } from '@chakra-ui/react';
import Icons from '~/services/Icons';
import React from 'react';
import { useNavigate } from 'react-router';

const Back = () => {
    const navigate = useNavigate();
    const history = window.history;

    return (
        <>
            {history.length > 1 ? (
                <Tooltip label={'Back'}>
                    <IconButton
                        icon={<Icon as={Icons.Back} />}
                        fontSize={'xl'}
                        aria-label={'back'}
                        size='lg'
                        colorScheme={'purple'}
                        variant={'outline'}
                        onClick={() => navigate(-1)}
                    />
                </Tooltip>
            ) : (
                ''
            )}
        </>
    );
};

export default Back;