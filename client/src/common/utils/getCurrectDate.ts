import dayjs from 'dayjs';

const getCorrentDate = (date: Date): Date => {
    const timezomeOffset = date.getTimezoneOffset() * 60000;
    return dayjs(Number(date) - timezomeOffset).toDate();
};

export default getCorrentDate;
