#[doc = "Register `ICWUR2` reader"]
pub struct R(crate::R<ICWUR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICWUR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICWUR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICWUR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICWUR2` writer"]
pub struct W(crate::W<ICWUR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICWUR2_SPEC>;
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
impl From<crate::W<ICWUR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICWUR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUSEN` reader - Wake-up Function Synchronous Enable"]
pub type WUSEN_R = crate::BitReader<WUSEN_A>;
#[doc = "Wake-up Function Synchronous Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUSEN_A {
    #[doc = "0: IIC asynchronous circuit enable"]
    _0 = 0,
    #[doc = "1: IIC synchronous circuit enable"]
    _1 = 1,
}
impl From<WUSEN_A> for bool {
    #[inline(always)]
    fn from(variant: WUSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WUSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUSEN_A {
        match self.bits {
            false => WUSEN_A::_0,
            true => WUSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUSEN_A::_1
    }
}
#[doc = "Field `WUSEN` writer - Wake-up Function Synchronous Enable"]
pub type WUSEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICWUR2_SPEC, WUSEN_A, O>;
impl<'a, const O: u8> WUSEN_W<'a, O> {
    #[doc = "IIC asynchronous circuit enable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUSEN_A::_0)
    }
    #[doc = "IIC synchronous circuit enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUSEN_A::_1)
    }
}
#[doc = "Field `WUASYF` reader - Wake-up Function Asynchronous Operation Status Flag"]
pub type WUASYF_R = crate::BitReader<WUASYF_A>;
#[doc = "Wake-up Function Asynchronous Operation Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUASYF_A {
    #[doc = "0: IIC synchronous circuit enable condition"]
    _0 = 0,
    #[doc = "1: IIC asynchronous circuit enable condition."]
    _1 = 1,
}
impl From<WUASYF_A> for bool {
    #[inline(always)]
    fn from(variant: WUASYF_A) -> Self {
        variant as u8 != 0
    }
}
impl WUASYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUASYF_A {
        match self.bits {
            false => WUASYF_A::_0,
            true => WUASYF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUASYF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUASYF_A::_1
    }
}
#[doc = "Field `WUSYF` reader - Wake-up Function Synchronous Operation Status Flag"]
pub type WUSYF_R = crate::BitReader<WUSYF_A>;
#[doc = "Wake-up Function Synchronous Operation Status Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUSYF_A {
    #[doc = "0: IIC asynchronous circuit enable condition"]
    _0 = 0,
    #[doc = "1: IIC synchronous circuit enable condition."]
    _1 = 1,
}
impl From<WUSYF_A> for bool {
    #[inline(always)]
    fn from(variant: WUSYF_A) -> Self {
        variant as u8 != 0
    }
}
impl WUSYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUSYF_A {
        match self.bits {
            false => WUSYF_A::_0,
            true => WUSYF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUSYF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUSYF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up Function Synchronous Enable"]
    #[inline(always)]
    pub fn wusen(&self) -> WUSEN_R {
        WUSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up Function Asynchronous Operation Status Flag"]
    #[inline(always)]
    pub fn wuasyf(&self) -> WUASYF_R {
        WUASYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up Function Synchronous Operation Status Flag"]
    #[inline(always)]
    pub fn wusyf(&self) -> WUSYF_R {
        WUSYF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Function Synchronous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wusen(&mut self) -> WUSEN_W<0> {
        WUSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Wake up Unit Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icwur2](index.html) module"]
pub struct ICWUR2_SPEC;
impl crate::RegisterSpec for ICWUR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [icwur2::R](R) reader structure"]
impl crate::Readable for ICWUR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icwur2::W](W) writer structure"]
impl crate::Writable for ICWUR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICWUR2 to value 0xfd"]
impl crate::Resettable for ICWUR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xfd;
}
