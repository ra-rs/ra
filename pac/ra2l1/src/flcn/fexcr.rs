#[doc = "Register `FEXCR` reader"]
pub struct R(crate::R<FEXCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEXCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEXCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEXCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEXCR` writer"]
pub struct W(crate::W<FEXCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEXCR_SPEC>;
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
impl From<crate::W<FEXCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEXCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD` reader - Software Command Setting"]
pub type CMD_R = crate::FieldReader<u8, CMD_A>;
#[doc = "Software Command Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "2: Access window information program Startup area selection and security setting"]
    _010 = 2,
    #[doc = "3: OCDID1 program"]
    _011 = 3,
    #[doc = "4: OCDID2 program"]
    _100 = 4,
    #[doc = "5: OCDID3 program"]
    _101 = 5,
    #[doc = "6: OCDID4 program"]
    _110 = 6,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            2 => Some(CMD_A::_010),
            3 => Some(CMD_A::_011),
            4 => Some(CMD_A::_100),
            5 => Some(CMD_A::_101),
            6 => Some(CMD_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CMD_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CMD_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CMD_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CMD_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CMD_A::_110
    }
}
#[doc = "Field `CMD` writer - Software Command Setting"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FEXCR_SPEC, u8, CMD_A, 3, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "Access window information program Startup area selection and security setting"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CMD_A::_010)
    }
    #[doc = "OCDID1 program"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CMD_A::_011)
    }
    #[doc = "OCDID2 program"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CMD_A::_100)
    }
    #[doc = "OCDID3 program"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CMD_A::_101)
    }
    #[doc = "OCDID4 program"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CMD_A::_110)
    }
}
#[doc = "Field `OPST` reader - Processing Start"]
pub type OPST_R = crate::BitReader<OPST_A>;
#[doc = "Processing Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPST_A {
    #[doc = "0: Processing stops"]
    _0 = 0,
    #[doc = "1: Processing starts."]
    _1 = 1,
}
impl From<OPST_A> for bool {
    #[inline(always)]
    fn from(variant: OPST_A) -> Self {
        variant as u8 != 0
    }
}
impl OPST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPST_A {
        match self.bits {
            false => OPST_A::_0,
            true => OPST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OPST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OPST_A::_1
    }
}
#[doc = "Field `OPST` writer - Processing Start"]
pub type OPST_W<'a, const O: u8> = crate::BitWriter<'a, u8, FEXCR_SPEC, OPST_A, O>;
impl<'a, const O: u8> OPST_W<'a, O> {
    #[doc = "Processing stops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OPST_A::_0)
    }
    #[doc = "Processing starts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OPST_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Software Command Setting"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - Processing Start"]
    #[inline(always)]
    pub fn opst(&self) -> OPST_R {
        OPST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Software Command Setting"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Bit 7 - Processing Start"]
    #[inline(always)]
    #[must_use]
    pub fn opst(&mut self) -> OPST_W<7> {
        OPST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Extra Area Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fexcr](index.html) module"]
pub struct FEXCR_SPEC;
impl crate::RegisterSpec for FEXCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fexcr::R](R) reader structure"]
impl crate::Readable for FEXCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fexcr::W](W) writer structure"]
impl crate::Writable for FEXCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEXCR to value 0"]
impl crate::Resettable for FEXCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
