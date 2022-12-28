#[doc = "Register `CS%sMOD` reader"]
pub struct R(crate::R<CSMOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSMOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSMOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSMOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS%sMOD` writer"]
pub struct W(crate::W<CSMOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSMOD_SPEC>;
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
impl From<crate::W<CSMOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSMOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRMOD` reader - Write Access Mode Select"]
pub type WRMOD_R = crate::BitReader<WRMOD_A>;
#[doc = "Write Access Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRMOD_A {
    #[doc = "0: Byte strobe mode"]
    _0 = 0,
    #[doc = "1: Single write strobe mode"]
    _1 = 1,
}
impl From<WRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: WRMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl WRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRMOD_A {
        match self.bits {
            false => WRMOD_A::_0,
            true => WRMOD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WRMOD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WRMOD_A::_1
    }
}
#[doc = "Field `WRMOD` writer - Write Access Mode Select"]
pub type WRMOD_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSMOD_SPEC, WRMOD_A, O>;
impl<'a, const O: u8> WRMOD_W<'a, O> {
    #[doc = "Byte strobe mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WRMOD_A::_0)
    }
    #[doc = "Single write strobe mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WRMOD_A::_1)
    }
}
#[doc = "Field `EWENB` reader - External Wait Enable"]
pub type EWENB_R = crate::BitReader<EWENB_A>;
#[doc = "External Wait Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWENB_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<EWENB_A> for bool {
    #[inline(always)]
    fn from(variant: EWENB_A) -> Self {
        variant as u8 != 0
    }
}
impl EWENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWENB_A {
        match self.bits {
            false => EWENB_A::_0,
            true => EWENB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWENB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWENB_A::_1
    }
}
#[doc = "Field `EWENB` writer - External Wait Enable"]
pub type EWENB_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSMOD_SPEC, EWENB_A, O>;
impl<'a, const O: u8> EWENB_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EWENB_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EWENB_A::_1)
    }
}
#[doc = "Field `PRENB` reader - Page Read Access Enable"]
pub type PRENB_R = crate::BitReader<PRENB_A>;
#[doc = "Page Read Access Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRENB_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<PRENB_A> for bool {
    #[inline(always)]
    fn from(variant: PRENB_A) -> Self {
        variant as u8 != 0
    }
}
impl PRENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRENB_A {
        match self.bits {
            false => PRENB_A::_0,
            true => PRENB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRENB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRENB_A::_1
    }
}
#[doc = "Field `PRENB` writer - Page Read Access Enable"]
pub type PRENB_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSMOD_SPEC, PRENB_A, O>;
impl<'a, const O: u8> PRENB_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRENB_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRENB_A::_1)
    }
}
#[doc = "Field `PWENB` reader - Page Write Access Enable"]
pub type PWENB_R = crate::BitReader<PWENB_A>;
#[doc = "Page Write Access Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWENB_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<PWENB_A> for bool {
    #[inline(always)]
    fn from(variant: PWENB_A) -> Self {
        variant as u8 != 0
    }
}
impl PWENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWENB_A {
        match self.bits {
            false => PWENB_A::_0,
            true => PWENB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWENB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWENB_A::_1
    }
}
#[doc = "Field `PWENB` writer - Page Write Access Enable"]
pub type PWENB_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSMOD_SPEC, PWENB_A, O>;
impl<'a, const O: u8> PWENB_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWENB_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWENB_A::_1)
    }
}
#[doc = "Field `PRDMOD` reader - Page Read Access Mode Select"]
pub type PRDMOD_R = crate::BitReader<PRDMOD_A>;
#[doc = "Page Read Access Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRDMOD_A {
    #[doc = "0: Normal access compatible mode"]
    _0 = 0,
    #[doc = "1: External data read continuous assertion mode"]
    _1 = 1,
}
impl From<PRDMOD_A> for bool {
    #[inline(always)]
    fn from(variant: PRDMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl PRDMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRDMOD_A {
        match self.bits {
            false => PRDMOD_A::_0,
            true => PRDMOD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRDMOD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRDMOD_A::_1
    }
}
#[doc = "Field `PRDMOD` writer - Page Read Access Mode Select"]
pub type PRDMOD_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSMOD_SPEC, PRDMOD_A, O>;
impl<'a, const O: u8> PRDMOD_W<'a, O> {
    #[doc = "Normal access compatible mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRDMOD_A::_0)
    }
    #[doc = "External data read continuous assertion mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRDMOD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Write Access Mode Select"]
    #[inline(always)]
    pub fn wrmod(&self) -> WRMOD_R {
        WRMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - External Wait Enable"]
    #[inline(always)]
    pub fn ewenb(&self) -> EWENB_R {
        EWENB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Page Read Access Enable"]
    #[inline(always)]
    pub fn prenb(&self) -> PRENB_R {
        PRENB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Page Write Access Enable"]
    #[inline(always)]
    pub fn pwenb(&self) -> PWENB_R {
        PWENB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Page Read Access Mode Select"]
    #[inline(always)]
    pub fn prdmod(&self) -> PRDMOD_R {
        PRDMOD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Access Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn wrmod(&mut self) -> WRMOD_W<0> {
        WRMOD_W::new(self)
    }
    #[doc = "Bit 3 - External Wait Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewenb(&mut self) -> EWENB_W<3> {
        EWENB_W::new(self)
    }
    #[doc = "Bit 8 - Page Read Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prenb(&mut self) -> PRENB_W<8> {
        PRENB_W::new(self)
    }
    #[doc = "Bit 9 - Page Write Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwenb(&mut self) -> PWENB_W<9> {
        PWENB_W::new(self)
    }
    #[doc = "Bit 15 - Page Read Access Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn prdmod(&mut self) -> PRDMOD_W<15> {
        PRDMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS%s Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csmod](index.html) module"]
pub struct CSMOD_SPEC;
impl crate::RegisterSpec for CSMOD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csmod::R](R) reader structure"]
impl crate::Readable for CSMOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csmod::W](W) writer structure"]
impl crate::Writable for CSMOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS%sMOD to value 0"]
impl crate::Resettable for CSMOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
