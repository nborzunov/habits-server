import { useRecoilValue } from 'recoil';
import { activeUserState } from '~/common/store/atoms';
import { Avatar, Stack, Text } from '@chakra-ui/react';
import React from 'react';

const ProfileInfo = () => {
    const user = useRecoilValue(activeUserState);

    return (
        <Stack
            direction={'row'}
            spacing={3}
            align={'center'}
            px='4'
            mx='4'
            rounded='md'
            py='3'
            cursor='pointer'
            color='gray.600'
            _hover={{
                bg: 'purple.300',
                color: 'whiteAlpha.900',
            }}
            role='group'
            fontWeight='bold'
            transition='.15s ease'
        >
            <Avatar color={'black'} size={'sm'} colorScheme={'black'} bgColor={'purple.300'} />

            <Text>{user?.username}</Text>
        </Stack>
    );
};

export default ProfileInfo;
