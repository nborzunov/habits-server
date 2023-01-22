import { Box, Flex, Heading, Icon, IconButton, Select, Text } from '@chakra-ui/react';
import Icons from '~/common/helpers/Icons';

const Dashboard = () => {
    // TODO
    // Chart with done/all
    // Calendar with done/all

    return (
        <Box>
            <Flex justifyContent='space-between' p='6' py='8'>
                <Flex flexDir='column'>
                    <Heading fontSize='24px'>Hello, Nikolay👋</Heading>
                    <Text color='gray.500'>Let&apos;s check your stats today!</Text>
                </Flex>

                <Flex gap='2'>
                    <Select
                        placeholder='Select option'
                        variant='outline'
                        bg='white'
                        value='overall'
                    >
                        <option value='overall'>Overall</option>
                        <option value='option2'>Option 2</option>
                        <option value='option3'>Option 3</option>
                    </Select>

                    <IconButton
                        aria-label='Search database'
                        bg='white'
                        icon={<Icon as={Icons.Notifications} />}
                    />
                </Flex>
            </Flex>
        </Box>
    );
};

export default Dashboard;
