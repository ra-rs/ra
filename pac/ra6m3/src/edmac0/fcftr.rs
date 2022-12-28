#[doc = "Register `FCFTR` reader"]
pub struct R(crate::R<FCFTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFTR` writer"]
pub struct W(crate::W<FCFTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FCFTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFDO` reader - Receive FIFO Data PAUSE Output Threshold(When (RFDO+1)x256-32 bytes of data is stored in the receive FIFO.)"]
pub type RFDO_R = crate::FieldReader<u8, RFDO_A>;
#[doc = "Receive FIFO Data PAUSE Output Threshold(When (RFDO+1)x256-32 bytes of data is stored in the receive FIFO.)\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFDO_A {
    #[doc = "0: When 224 ( 256 - 32) bytes of data is stored in the receive FIFO."]
    _000 = 0,
    #[doc = "1: When 480 ( 512 - 32) bytes of data is stored in the receive FIFO."]
    _001 = 1,
    #[doc = "2: When 736 ( 768 - 32) bytes of data is stored in the receive FIFO."]
    _010 = 2,
    #[doc = "3: When 992 (1024 - 32) bytes of data is stored in the receive FIFO."]
    _011 = 3,
    #[doc = "4: When 1248 (1280 - 32) bytes of data is stored in the receive FIFO."]
    _100 = 4,
    #[doc = "5: When 1504 (1536 - 32) bytes of data is stored in the receive FIFO."]
    _101 = 5,
    #[doc = "6: When 1760 (1792 - 32) bytes of data is stored in the receive FIFO."]
    _110 = 6,
    #[doc = "7: When 2016 (2048 - 32) bytes of data is stored in the receive FIFO."]
    _111 = 7,
}
impl From<RFDO_A> for u8 {
    #[inline(always)]
    fn from(variant: RFDO_A) -> Self {
        variant as _
    }
}
impl RFDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDO_A {
        match self.bits {
            0 => RFDO_A::_000,
            1 => RFDO_A::_001,
            2 => RFDO_A::_010,
            3 => RFDO_A::_011,
            4 => RFDO_A::_100,
            5 => RFDO_A::_101,
            6 => RFDO_A::_110,
            7 => RFDO_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RFDO_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RFDO_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RFDO_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RFDO_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RFDO_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RFDO_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RFDO_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RFDO_A::_111
    }
}
#[doc = "Field `RFDO` writer - Receive FIFO Data PAUSE Output Threshold(When (RFDO+1)x256-32 bytes of data is stored in the receive FIFO.)"]
pub type RFDO_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FCFTR_SPEC, u8, RFDO_A, 3, O>;
impl<'a, const O: u8> RFDO_W<'a, O> {
    #[doc = "When 224 ( 256 - 32) bytes of data is stored in the receive FIFO."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RFDO_A::_000)
    }
    #[doc = "When 480 ( 512 - 32) bytes of data is stored in the receive FIFO."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RFDO_A::_001)
    }
    #[doc = "When 736 ( 768 - 32) bytes of data is stored in the receive FIFO."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(RFDO_A::_010)
    }
    #[doc = "When 992 (1024 - 32) bytes of data is stored in the receive FIFO."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(RFDO_A::_011)
    }
    #[doc = "When 1248 (1280 - 32) bytes of data is stored in the receive FIFO."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RFDO_A::_100)
    }
    #[doc = "When 1504 (1536 - 32) bytes of data is stored in the receive FIFO."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(RFDO_A::_101)
    }
    #[doc = "When 1760 (1792 - 32) bytes of data is stored in the receive FIFO."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(RFDO_A::_110)
    }
    #[doc = "When 2016 (2048 - 32) bytes of data is stored in the receive FIFO."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(RFDO_A::_111)
    }
}
#[doc = "Field `RFFO` reader - Receive FIFO Frame PAUSE Output Threshold(When ((RFFO+1)x2) receive frames have been stored in the receive FIFO.)"]
pub type RFFO_R = crate::FieldReader<u8, RFFO_A>;
#[doc = "Receive FIFO Frame PAUSE Output Threshold(When ((RFFO+1)x2) receive frames have been stored in the receive FIFO.)\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFFO_A {
    #[doc = "0: When 2 receive frames have been stored in the receive FIFO."]
    _000 = 0,
    #[doc = "1: When 4 receive frames have been stored in the receive FIFO."]
    _001 = 1,
    #[doc = "2: When 6 receive frames have been stored in the receive FIFO."]
    _010 = 2,
    #[doc = "3: When 8 receive frames have been stored in the receive FIFO."]
    _011 = 3,
    #[doc = "4: When 10 receive frames have been stored in the receive FIFO."]
    _100 = 4,
    #[doc = "5: When 12 receive frames have been stored in the receive FIFO."]
    _101 = 5,
    #[doc = "6: When 14 receive frames have been stored in the receive FIFO."]
    _110 = 6,
    #[doc = "7: When 16 receive frames have been stored in the receive FIFO."]
    _111 = 7,
}
impl From<RFFO_A> for u8 {
    #[inline(always)]
    fn from(variant: RFFO_A) -> Self {
        variant as _
    }
}
impl RFFO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFFO_A {
        match self.bits {
            0 => RFFO_A::_000,
            1 => RFFO_A::_001,
            2 => RFFO_A::_010,
            3 => RFFO_A::_011,
            4 => RFFO_A::_100,
            5 => RFFO_A::_101,
            6 => RFFO_A::_110,
            7 => RFFO_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RFFO_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RFFO_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RFFO_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RFFO_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RFFO_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RFFO_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RFFO_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RFFO_A::_111
    }
}
#[doc = "Field `RFFO` writer - Receive FIFO Frame PAUSE Output Threshold(When ((RFFO+1)x2) receive frames have been stored in the receive FIFO.)"]
pub type RFFO_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FCFTR_SPEC, u8, RFFO_A, 3, O>;
impl<'a, const O: u8> RFFO_W<'a, O> {
    #[doc = "When 2 receive frames have been stored in the receive FIFO."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RFFO_A::_000)
    }
    #[doc = "When 4 receive frames have been stored in the receive FIFO."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RFFO_A::_001)
    }
    #[doc = "When 6 receive frames have been stored in the receive FIFO."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(RFFO_A::_010)
    }
    #[doc = "When 8 receive frames have been stored in the receive FIFO."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(RFFO_A::_011)
    }
    #[doc = "When 10 receive frames have been stored in the receive FIFO."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RFFO_A::_100)
    }
    #[doc = "When 12 receive frames have been stored in the receive FIFO."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(RFFO_A::_101)
    }
    #[doc = "When 14 receive frames have been stored in the receive FIFO."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(RFFO_A::_110)
    }
    #[doc = "When 16 receive frames have been stored in the receive FIFO."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(RFFO_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive FIFO Data PAUSE Output Threshold(When (RFDO+1)x256-32 bytes of data is stored in the receive FIFO.)"]
    #[inline(always)]
    pub fn rfdo(&self) -> RFDO_R {
        RFDO_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - Receive FIFO Frame PAUSE Output Threshold(When ((RFFO+1)x2) receive frames have been stored in the receive FIFO.)"]
    #[inline(always)]
    pub fn rffo(&self) -> RFFO_R {
        RFFO_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive FIFO Data PAUSE Output Threshold(When (RFDO+1)x256-32 bytes of data is stored in the receive FIFO.)"]
    #[inline(always)]
    #[must_use]
    pub fn rfdo(&mut self) -> RFDO_W<0> {
        RFDO_W::new(self)
    }
    #[doc = "Bits 16:18 - Receive FIFO Frame PAUSE Output Threshold(When ((RFFO+1)x2) receive frames have been stored in the receive FIFO.)"]
    #[inline(always)]
    #[must_use]
    pub fn rffo(&mut self) -> RFFO_W<16> {
        RFFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flow Control Start FIFO Threshold Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcftr](index.html) module"]
pub struct FCFTR_SPEC;
impl crate::RegisterSpec for FCFTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcftr::R](R) reader structure"]
impl crate::Readable for FCFTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcftr::W](W) writer structure"]
impl crate::Writable for FCFTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCFTR to value 0x0007_0007"]
impl crate::Resettable for FCFTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0007;
}
