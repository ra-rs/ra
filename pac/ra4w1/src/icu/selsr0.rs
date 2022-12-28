#[doc = "Register `SELSR0` reader"]
pub struct R(crate::R<SELSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SELSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SELSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SELSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SELSR0` writer"]
pub struct W(crate::W<SELSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SELSR0_SPEC>;
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
impl From<crate::W<SELSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SELSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELS` reader - SYS Event Link Select"]
pub type SELS_R = crate::FieldReader<u8, SELS_A>;
#[doc = "SYS Event Link Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELS_A {
    #[doc = "0: Nothing is selected"]
    _0X000 = 0,
    #[doc = "21: DTC_COMPLETE"]
    _0X015 = 21,
    #[doc = "45: ADC140_WCMPM"]
    _0X02D = 45,
    #[doc = "46: ADC140_WCMPUM"]
    _0X02E = 46,
    #[doc = "72: CTSU_CTSUFN"]
    _0X048 = 72,
    #[doc = "74: DOC_DOPCI"]
    _0X04A = 74,
    #[doc = "176: SCI0_AM"]
    _0X0B0 = 176,
    #[doc = "177: SCI0_RXI_OR_ERI"]
    _0X0B1 = 177,
}
impl From<SELS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELS_A) -> Self {
        variant as _
    }
}
impl SELS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELS_A> {
        match self.bits {
            0 => Some(SELS_A::_0X000),
            21 => Some(SELS_A::_0X015),
            45 => Some(SELS_A::_0X02D),
            46 => Some(SELS_A::_0X02E),
            72 => Some(SELS_A::_0X048),
            74 => Some(SELS_A::_0X04A),
            176 => Some(SELS_A::_0X0B0),
            177 => Some(SELS_A::_0X0B1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X000`"]
    #[inline(always)]
    pub fn is_0x000(&self) -> bool {
        *self == SELS_A::_0X000
    }
    #[doc = "Checks if the value of the field is `_0X015`"]
    #[inline(always)]
    pub fn is_0x015(&self) -> bool {
        *self == SELS_A::_0X015
    }
    #[doc = "Checks if the value of the field is `_0X02D`"]
    #[inline(always)]
    pub fn is_0x02d(&self) -> bool {
        *self == SELS_A::_0X02D
    }
    #[doc = "Checks if the value of the field is `_0X02E`"]
    #[inline(always)]
    pub fn is_0x02e(&self) -> bool {
        *self == SELS_A::_0X02E
    }
    #[doc = "Checks if the value of the field is `_0X048`"]
    #[inline(always)]
    pub fn is_0x048(&self) -> bool {
        *self == SELS_A::_0X048
    }
    #[doc = "Checks if the value of the field is `_0X04A`"]
    #[inline(always)]
    pub fn is_0x04a(&self) -> bool {
        *self == SELS_A::_0X04A
    }
    #[doc = "Checks if the value of the field is `_0X0B0`"]
    #[inline(always)]
    pub fn is_0x0b0(&self) -> bool {
        *self == SELS_A::_0X0B0
    }
    #[doc = "Checks if the value of the field is `_0X0B1`"]
    #[inline(always)]
    pub fn is_0x0b1(&self) -> bool {
        *self == SELS_A::_0X0B1
    }
}
#[doc = "Field `SELS` writer - SYS Event Link Select"]
pub type SELS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SELSR0_SPEC, u8, SELS_A, 8, O>;
impl<'a, const O: u8> SELS_W<'a, O> {
    #[doc = "Nothing is selected"]
    #[inline(always)]
    pub fn _0x000(self) -> &'a mut W {
        self.variant(SELS_A::_0X000)
    }
    #[doc = "DTC_COMPLETE"]
    #[inline(always)]
    pub fn _0x015(self) -> &'a mut W {
        self.variant(SELS_A::_0X015)
    }
    #[doc = "ADC140_WCMPM"]
    #[inline(always)]
    pub fn _0x02d(self) -> &'a mut W {
        self.variant(SELS_A::_0X02D)
    }
    #[doc = "ADC140_WCMPUM"]
    #[inline(always)]
    pub fn _0x02e(self) -> &'a mut W {
        self.variant(SELS_A::_0X02E)
    }
    #[doc = "CTSU_CTSUFN"]
    #[inline(always)]
    pub fn _0x048(self) -> &'a mut W {
        self.variant(SELS_A::_0X048)
    }
    #[doc = "DOC_DOPCI"]
    #[inline(always)]
    pub fn _0x04a(self) -> &'a mut W {
        self.variant(SELS_A::_0X04A)
    }
    #[doc = "SCI0_AM"]
    #[inline(always)]
    pub fn _0x0b0(self) -> &'a mut W {
        self.variant(SELS_A::_0X0B0)
    }
    #[doc = "SCI0_RXI_OR_ERI"]
    #[inline(always)]
    pub fn _0x0b1(self) -> &'a mut W {
        self.variant(SELS_A::_0X0B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - SYS Event Link Select"]
    #[inline(always)]
    pub fn sels(&self) -> SELS_R {
        SELS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SYS Event Link Select"]
    #[inline(always)]
    #[must_use]
    pub fn sels(&mut self) -> SELS_W<0> {
        SELS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Snooze Event Link Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [selsr0](index.html) module"]
pub struct SELSR0_SPEC;
impl crate::RegisterSpec for SELSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [selsr0::R](R) reader structure"]
impl crate::Readable for SELSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [selsr0::W](W) writer structure"]
impl crate::Writable for SELSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SELSR0 to value 0"]
impl crate::Resettable for SELSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
